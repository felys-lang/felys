use crate::ast::{AssOp, BinOp, Block, Bool, Chunk, Expr, Lit, Pat, Stmt};
use crate::cyrene::ir::{Const, Context, Dst, Id, Instruction, Label, Var};
use crate::cyrene::meta::Meta;
use crate::demiurge::Function;
use crate::error::Fault;

type Stack = Vec<(Label, Label, Option<Option<Id>>)>;

impl Block {
    pub fn build<'a>(
        &self,
        args: impl Iterator<Item=&'a usize>,
        meta: &Meta,
    ) -> Result<Function, Fault> {
        let mut stk = Vec::new();
        let mut ctx = Context::default();
        ctx.seal(Label::Entry)?;
        for id in args {
            let var = ctx.var();
            ctx.define(ctx.cursor, Id::Interned(*id), var);
        }

        if let Ok(var) = self.ir(&mut ctx, &mut stk, meta)?.var() {
            ctx.define(ctx.cursor, Id::Ret, var);
        }
        ctx.jump(Label::Exit);
        ctx.seal(Label::Exit)?;

        ctx.cursor = Label::Exit;
        let var = ctx.lookup(ctx.cursor, Id::Ret)?;
        ctx.ret(var);
        Ok(ctx.export())
    }

    fn ir(&self, ctx: &mut Context, stk: &mut Stack, meta: &Meta) -> Result<Dst, Fault> {
        let mut iter = self.0.iter().peekable();
        let mut result = Ok(Dst::void());
        while let Some(stmt) = iter.next() {
            let ret = stmt.ir(ctx, stk, meta)?;
            if ret.var().is_ok() {
                if iter.peek().is_none() {
                    result = Ok(ret);
                } else {
                    result = Err(Fault::BlockEarlyEnd);
                };
                break;
            }
        }
        result
    }
}

impl Stmt {
    fn ir(&self, ctx: &mut Context, stk: &mut Stack, meta: &Meta) -> Result<Dst, Fault> {
        match self {
            Stmt::Empty => Ok(Dst::void()),
            Stmt::Expr(expr) => expr.ir(ctx, stk, meta),
            Stmt::Semi(expr) => expr.ir(ctx, stk, meta).and(Ok(Dst::void())),
            Stmt::Assign(pat, op, expr) => {
                let op = match op {
                    AssOp::AddEq => Some(BinOp::Add),
                    AssOp::SubEq => Some(BinOp::Sub),
                    AssOp::MulEq => Some(BinOp::Mul),
                    AssOp::DivEq => Some(BinOp::Div),
                    AssOp::ModEq => Some(BinOp::Mod),
                    AssOp::Eq => None,
                };
                let var = expr.ir(ctx, stk, meta)?.var()?;
                pat.ir(ctx, &op, var)?;
                Ok(Dst::void())
            }
        }
    }
}

impl Pat {
    fn ir(&self, ctx: &mut Context, op: &Option<BinOp>, mut rhs: Var) -> Result<(), Fault> {
        match self {
            Pat::Any => {}
            Pat::Tuple(pats) => {
                for (i, pat) in pats.iter().enumerate() {
                    let field = ctx.var();
                    ctx.push(Instruction::Field(field, rhs, i));
                    pat.ir(ctx, op, field)?
                }
            }
            Pat::Ident(id) => {
                let id = Id::Interned(*id);
                if let Some(bop) = op {
                    let lhs = ctx.lookup(ctx.cursor, id)?;
                    let var = ctx.var();
                    ctx.push(Instruction::Binary(var, lhs, bop.clone(), rhs));
                    rhs = var;
                }
                ctx.define(ctx.cursor, id, rhs)
            }
        }
        Ok(())
    }
}

impl Expr {
    fn ir(&self, ctx: &mut Context, stk: &mut Stack, meta: &Meta) -> Result<Dst, Fault> {
        match self {
            Expr::Block(block) => block.ir(ctx, stk, meta),
            Expr::Break(expr) => {
                let expr = expr.as_ref().map(|x| x.ir(ctx, stk, meta));
                let (_, end, wb) = stk.last_mut().ok_or(Fault::OutsideLoop)?;
                match (expr, wb) {
                    (None, None) | (None, Some(None)) => {}
                    (Some(x), Some(wb)) if wb.is_none() => {
                        let id = ctx.id();
                        *wb = Some(id);
                        ctx.define(ctx.cursor, id, x?.var()?);
                    }
                    (Some(x), Some(Some(id))) => {
                        ctx.define(ctx.cursor, *id, x?.var()?);
                    }
                    _ => return Err(Fault::UndeterminedValue),
                }
                ctx.jump(*end);
                ctx.unreachable()
            }
            Expr::Continue => {
                let (start, _, _) = stk.last().ok_or(Fault::OutsideLoop)?;
                ctx.jump(*start);
                ctx.unreachable()
            }
            Expr::For(_, _, _) => Err(Fault::NotImplemented),
            Expr::If(expr, block, alter) => {
                let then = ctx.label();
                let otherwise = ctx.label();
                let join = ctx.label();
                let mut ret = Err(Fault::UndeterminedValue);

                ctx.add(then);
                ctx.add(otherwise);
                ctx.add(join);

                let cond = expr.ir(ctx, stk, meta)?.var()?;
                ctx.branch(cond, then, otherwise);
                ctx.seal(then)?;
                ctx.seal(otherwise)?;

                ctx.cursor = then;
                if let Ok(var) = block.ir(ctx, stk, meta)?.var() {
                    let id = ctx.id();
                    ret = Ok(id);
                    ctx.define(ctx.cursor, id, var);
                }
                ctx.jump(join);

                ctx.cursor = otherwise;
                let mut returned = false;
                if let Some(alt) = alter
                    && let Ok(var) = alt.ir(ctx, stk, meta)?.var()
                {
                    ctx.define(ctx.cursor, ret.clone()?, var);
                    returned = true;
                }
                ctx.jump(join);
                ctx.seal(join)?;

                ctx.cursor = join;
                if returned {
                    Ok(ctx.lookup(ctx.cursor, ret.unwrap())?.into())
                } else {
                    Ok(Dst::void())
                }
            }
            Expr::Loop(block) => {
                let body = ctx.label();
                let end = ctx.label();

                ctx.add(body);
                ctx.add(end);

                ctx.jump(body);

                ctx.cursor = body;
                stk.push((body, end, Some(None)));
                block.ir(ctx, stk, meta)?;
                let wb = stk.pop().unwrap().2.unwrap();
                ctx.jump(body);
                ctx.seal(body)?;
                ctx.seal(end)?;

                ctx.cursor = end;
                if let Some(id) = wb {
                    let var = ctx.lookup(ctx.cursor, id)?;
                    Ok(var.into())
                } else {
                    Ok(Dst::void())
                }
            }
            Expr::Return(expr) => {
                let var = expr.ir(ctx, stk, meta)?.var()?;
                ctx.define(ctx.cursor, Id::Ret, var);
                ctx.jump(Label::Exit);
                ctx.unreachable()
            }
            Expr::While(expr, block) => {
                let header = ctx.label();
                let body = ctx.label();
                let end = ctx.label();

                ctx.add(header);
                ctx.add(body);
                ctx.add(end);

                ctx.jump(header);

                ctx.cursor = header;
                let cond = expr.ir(ctx, stk, meta)?.var()?;
                ctx.branch(cond, body, end);
                ctx.seal(body)?;

                ctx.cursor = body;
                stk.push((header, end, None));
                block.ir(ctx, stk, meta)?;
                stk.pop();
                ctx.jump(header);
                ctx.seal(header)?;
                ctx.seal(end)?;

                ctx.cursor = end;
                Ok(Dst::void())
            }
            Expr::Binary(lhs, op, rhs) => {
                let l = lhs.ir(ctx, stk, meta)?.var()?;
                let r = rhs.ir(ctx, stk, meta)?.var()?;
                let var = ctx.var();
                ctx.push(Instruction::Binary(var, l, op.clone(), r));
                Ok(var.into())
            }
            Expr::Call(expr, args) => {
                let callable = expr.ir(ctx, stk, meta)?.var()?;
                let mut params = Vec::new();
                if let Some(args) = args {
                    for arg in args.iter() {
                        let param = arg.ir(ctx, stk, meta)?.var()?;
                        params.push(param);
                    }
                }
                let var = ctx.var();
                ctx.push(Instruction::Call(var, callable, params));
                Ok(var.into())
            }
            Expr::Field(expr, id) => {
                let src = expr.ir(ctx, stk, meta)?.var()?;
                let var = ctx.var();
                ctx.push(Instruction::Field(var, src, *id));
                Ok(var.into())
            }
            Expr::Method(expr, id, args) => {
                let src = expr.ir(ctx, stk, meta)?.var()?;
                let mut params = Vec::new();
                if let Some(args) = args {
                    for arg in args.iter() {
                        let param = arg.ir(ctx, stk, meta)?.var()?;
                        params.push(param);
                    }
                }
                let var = ctx.var();
                ctx.push(Instruction::Method(var, src, *id, params));
                Ok(var.into())
            }
            Expr::Index(expr, index) => {
                let src = expr.ir(ctx, stk, meta)?.var()?;
                let idx = index.ir(ctx, stk, meta)?.var()?;
                let var = ctx.var();
                ctx.push(Instruction::Index(var, src, idx));
                Ok(var.into())
            }
            Expr::Tuple(args) => {
                let mut params = Vec::new();
                for arg in args.iter() {
                    let param = arg.ir(ctx, stk, meta)?.var()?;
                    params.push(param);
                }
                let var = ctx.var();
                ctx.push(Instruction::Tuple(var, params));
                Ok(var.into())
            }
            Expr::List(args) => {
                let mut params = Vec::new();
                if let Some(args) = args {
                    for arg in args.iter() {
                        let param = arg.ir(ctx, stk, meta)?.var()?;
                        params.push(param);
                    }
                }
                let var = ctx.var();
                ctx.push(Instruction::List(var, params));
                Ok(var.into())
            }
            Expr::Lit(lit) => lit.ir(ctx, meta),
            Expr::Paren(expr) => expr.ir(ctx, stk, meta),
            Expr::Unary(op, inner) => {
                let i = inner.ir(ctx, stk, meta)?.var()?;
                let var = ctx.var();
                ctx.push(Instruction::Unary(var, op.clone(), i));
                Ok(var.into())
            }
            Expr::Path(path) => {
                if path.len() == 1 {
                    let id = Id::Interned(path.buffer()[0]);
                    if let Ok(var) = ctx.lookup(ctx.cursor, id) {
                        return Ok(var.into());
                    }
                }
                let var = ctx.var();
                if let Ok(id) = meta.constructors.get(path.iter()) {
                    ctx.push(Instruction::Group(var, id));
                } else {
                    let id = meta.ns.get(path.iter())?;
                    ctx.push(Instruction::Func(var, id));
                }
                Ok(var.into())
            }
        }
    }
}

impl Lit {
    fn ir(&self, ctx: &mut Context, meta: &Meta) -> Result<Dst, Fault> {
        let var = ctx.var();
        if let Some(c) = ctx.cache.get(self) {
            ctx.push(Instruction::Load(var, c.clone()));
            return Ok(var.into());
        }
        let c = match self {
            Lit::Int(x) => {
                let value = meta
                    .intern
                    .get(x)
                    .ok_or(Fault::InvalidConstant)?
                    .parse()
                    .map_err(|_| Fault::InvalidConstant)?;
                Const::Int(value)
            }
            Lit::Float(x) => {
                let value = meta
                    .intern
                    .get(x)
                    .ok_or(Fault::InvalidConstant)?
                    .parse::<f64>()
                    .map_err(|_| Fault::InvalidConstant)?
                    .to_bits();
                Const::Float(value)
            }
            Lit::Bool(x) => match x {
                Bool::True => Const::Bool(true),
                Bool::False => Const::Bool(false),
            },
            Lit::Str(x) => {
                let mut value = String::new();
                for chunk in x {
                    match chunk {
                        Chunk::Slice(x) => {
                            let s = meta.intern.get(x).unwrap();
                            value.push_str(s);
                        }
                        Chunk::Unicode(x) => {
                            let hex = meta.intern.get(x).unwrap();
                            let c = u32::from_str_radix(hex, 16)
                                .ok()
                                .and_then(char::from_u32)
                                .ok_or(Fault::InvalidConstant)?;
                            value.push(c)
                        }
                        Chunk::Escape(x) => {
                            let str = meta.intern.get(x).unwrap();
                            let c = match str {
                                "\'" => '\'',
                                "\"" => '"',
                                "n" => '\n',
                                "t" => '\t',
                                "r" => '\r',
                                "\\" => '\\',
                                _ => return Err(Fault::InvalidConstant),
                            };
                            value.push(c)
                        }
                    }
                }
                Const::Str(value.into())
            }
        };
        ctx.push(Instruction::Load(var, c));
        Ok(var.into())
    }
}
