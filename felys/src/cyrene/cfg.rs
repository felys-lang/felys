use crate::cyrene::context::{Context, Id};
use crate::cyrene::fault::Fault;
use crate::cyrene::meta::Meta;
use crate::utils::ast::{AssOp, BinOp, Block, Bool, Chunk, Expr, Lit, Pat, Path, Stmt};
use crate::utils::function::Function;
use crate::utils::ir::{Const, Instruction, Label, Var};

type Stack = Vec<(Label, Label, Option<(Id, Option<bool>)>)>;

impl Block {
    pub fn function(&self, args: Vec<usize>, meta: &mut Meta) -> Result<Function, Fault> {
        let mut stk = Vec::new();
        let mut ctx = Context::new(args.len());
        ctx.seal(Label::Entry)?;
        for (i, id) in args.iter().enumerate() {
            let var = ctx.var();
            ctx.push(Instruction::Arg(var, i));
            ctx.define(ctx.cursor, Id::Interned(*id), var);
        }

        if let Some(var) = self.ir(&mut ctx, &mut stk, meta)? {
            ctx.define(ctx.cursor, Id::Ret, var);
        }
        ctx.jump(Label::Exit);
        ctx.seal(Label::Exit)?;

        ctx.cursor = Label::Exit;
        let var = ctx
            .lookup(ctx.cursor, Id::Ret)
            .ok_or(Fault::FunctionNoReturn(self.clone()))?;
        ctx.ret(var);
        Ok(ctx.into())
    }

    fn ir(&self, ctx: &mut Context, stk: &mut Stack, meta: &Meta) -> Result<Option<Var>, Fault> {
        let mut iter = self.0.iter().peekable();
        let mut result = Ok(None);
        let mut i = 1;
        while let Some(stmt) = iter.next() {
            let ret = stmt.ir(ctx, stk, meta)?;
            if ret.is_some() {
                if iter.peek().is_none() {
                    result = Ok(ret);
                } else {
                    result = Err(Fault::BlockEarlyReturn(self.clone(), i));
                };
                break;
            }
            i += 1;
        }
        result
    }
}

impl Stmt {
    fn ir(&self, ctx: &mut Context, stk: &mut Stack, meta: &Meta) -> Result<Option<Var>, Fault> {
        match self {
            Stmt::Empty => Ok(None),
            Stmt::Expr(expr) => expr.ir(ctx, stk, meta),
            Stmt::Semi(expr) => expr.ir(ctx, stk, meta).and(Ok(None)),
            Stmt::Assign(pat, op, expr) => {
                let op = match op {
                    AssOp::AddEq => Some(BinOp::Add),
                    AssOp::SubEq => Some(BinOp::Sub),
                    AssOp::MulEq => Some(BinOp::Mul),
                    AssOp::DivEq => Some(BinOp::Div),
                    AssOp::ModEq => Some(BinOp::Mod),
                    AssOp::Eq => None,
                };
                let var = expr
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(expr.clone().into()))?;
                pat.ir(ctx, &op, var)?;
                Ok(None)
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
                    ctx.push(Instruction::Unpack(field, rhs, i));
                    pat.ir(ctx, op, field)?
                }
            }
            Pat::Ident(x) => {
                let id = Id::Interned(*x);
                if let Some(bop) = op {
                    let lhs = ctx.lookup(ctx.cursor, id).unwrap();
                    let var = ctx.var();
                    ctx.push(Instruction::Binary(var, lhs, *bop, rhs));
                    rhs = var;
                }
                ctx.define(ctx.cursor, id, rhs)
            }
        }
        Ok(())
    }
}

impl Expr {
    fn ir(&self, ctx: &mut Context, stk: &mut Stack, meta: &Meta) -> Result<Option<Var>, Fault> {
        match self {
            Expr::Block(block) => block.ir(ctx, stk, meta),
            Expr::Break(expr) => {
                let (_, end, wb) = stk
                    .last()
                    .cloned()
                    .ok_or(Fault::OutsideLoop(self.clone()))?;
                if let Some((id, action)) = wb
                    && action.unwrap_or(true)
                {
                    if let Some(x) = expr
                        && let Some(var) = x.ir(ctx, stk, meta)?
                    {
                        stk.last_mut().unwrap().2.as_mut().unwrap().1 = Some(true);
                        ctx.define(ctx.cursor, id, var)
                    } else {
                        stk.last_mut().unwrap().2.as_mut().unwrap().1 = Some(false);
                    }
                }
                ctx.jump(end);
                Ok(None)
            }
            Expr::Continue => {
                let (start, _, _) = stk.last().ok_or(Fault::OutsideLoop(self.clone()))?;
                ctx.jump(*start);
                Ok(None)
            }
            Expr::For(_, _, _) => Ok(None),
            Expr::If(expr, block, alter) => {
                let then = ctx.label();
                let otherwise = ctx.label();
                let join = ctx.label();
                let ret = ctx.id();

                let cond = expr
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(expr.clone()))?;
                ctx.branch(cond, then, otherwise);
                ctx.seal(then)?;
                ctx.seal(otherwise)?;

                let mut returned = [false, false];
                let mut joined = [false, false];

                ctx.cursor = then;
                if let Some(var) = block.ir(ctx, stk, meta)? {
                    ctx.define(ctx.cursor, ret, var);
                    returned[0] = true;
                }
                joined[0] = ctx.jump(join);

                ctx.cursor = otherwise;
                if let Some(alt) = alter
                    && let Some(var) = alt.ir(ctx, stk, meta)?
                {
                    ctx.define(ctx.cursor, ret, var);
                    returned[1] = true;
                }
                joined[1] = ctx.jump(join);

                ctx.seal(join)?;

                ctx.cursor = join;
                if joined == returned && (returned[0] || returned[1]) {
                    let var = ctx.lookup(ctx.cursor, ret).unwrap();
                    Ok(Some(var))
                } else {
                    Ok(None)
                }
            }
            Expr::Loop(block) => {
                let body = ctx.label();
                let end = ctx.label();
                let ret = ctx.id();

                ctx.jump(body);

                ctx.cursor = body;
                stk.push((body, end, Some((ret, None))));
                block.ir(ctx, stk, meta)?;
                let action = stk.pop().unwrap().2.unwrap().1;

                ctx.jump(body);
                ctx.seal(body)?;
                ctx.seal(end)?;

                ctx.cursor = end;
                if action.unwrap_or(false) {
                    let var = ctx.lookup(ctx.cursor, ret).unwrap();
                    Ok(Some(var))
                } else {
                    Ok(None)
                }
            }
            Expr::Return(expr) => {
                let var = expr
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(expr.clone()))?;
                ctx.define(ctx.cursor, Id::Ret, var);
                ctx.jump(Label::Exit);
                Ok(None)
            }
            Expr::While(expr, block) => {
                let header = ctx.label();
                let body = ctx.label();
                let end = ctx.label();

                ctx.jump(header);

                ctx.cursor = header;
                let cond = expr
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(expr.clone()))?;
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
                Ok(None)
            }
            Expr::Binary(lhs, op, rhs) => {
                let l = lhs
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(lhs.clone()))?;
                let r = rhs
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(rhs.clone()))?;
                let var = ctx.var();
                ctx.push(Instruction::Binary(var, l, *op, r));
                Ok(var.into())
            }
            Expr::Call(expr, args) => {
                let callable = expr
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(expr.clone()))?;
                let mut params = Vec::new();
                if let Some(args) = args {
                    for arg in args.iter() {
                        let param = arg
                            .ir(ctx, stk, meta)?
                            .ok_or(Fault::NoReturnValue(arg.clone().into()))?;
                        params.push(param);
                    }
                }
                let var = ctx.var();
                ctx.push(Instruction::Call(var, callable, params));
                Ok(var.into())
            }
            Expr::Field(expr, id) => {
                let src = expr
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(expr.clone()))?;
                let var = ctx.var();
                ctx.push(Instruction::Field(var, src, *id));
                Ok(var.into())
            }
            Expr::Method(expr, id, args) => {
                let src = expr
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(expr.clone()))?;
                let mut params = Vec::new();
                if let Some(args) = args {
                    for arg in args.iter() {
                        let param = arg
                            .ir(ctx, stk, meta)?
                            .ok_or(Fault::NoReturnValue(arg.clone().into()))?;
                        params.push(param);
                    }
                }
                let var = ctx.var();
                ctx.push(Instruction::Method(var, src, *id, params));
                Ok(var.into())
            }
            Expr::Index(expr, index) => {
                let src = expr
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(expr.clone()))?;
                let idx = index
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(index.clone()))?;
                let var = ctx.var();
                ctx.push(Instruction::Index(var, src, idx));
                Ok(var.into())
            }
            Expr::Tuple(args) => {
                let mut params = Vec::new();
                for arg in args.iter() {
                    let param = arg
                        .ir(ctx, stk, meta)?
                        .ok_or(Fault::NoReturnValue(arg.clone().into()))?;
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
                        let param = arg
                            .ir(ctx, stk, meta)?
                            .ok_or(Fault::NoReturnValue(arg.clone().into()))?;
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
                let i = inner
                    .ir(ctx, stk, meta)?
                    .ok_or(Fault::NoReturnValue(inner.clone()))?;
                let var = ctx.var();
                ctx.push(Instruction::Unary(var, *op, i));
                Ok(var.into())
            }
            Expr::Path(path) => path.ir(ctx, meta),
        }
    }
}

impl Path {
    fn ir(&self, ctx: &mut Context, meta: &Meta) -> Result<Option<Var>, Fault> {
        if let Some(id) = self.ident()
            && let Some(var) = ctx.lookup(ctx.cursor, id)
        {
            return Ok(var.into());
        }

        let Some((ptr, id)) = meta.namespace.get(self.0.iter()) else {
            return Err(Fault::PathNotExist(self.clone()));
        };

        let var = ctx.var();
        ctx.push(Instruction::Pointer(var, ptr, id));
        Ok(var.into())
    }

    fn ident(&self) -> Option<Id> {
        if self.0.len() == 1 {
            Some(Id::Interned(self.0.buffer()[0]))
        } else {
            None
        }
    }
}

impl Lit {
    fn ir(&self, ctx: &mut Context, meta: &Meta) -> Result<Option<Var>, Fault> {
        let var = ctx.var();
        if let Some(c) = ctx.consts.get(self) {
            ctx.push(Instruction::Load(var, c.clone()));
            return Ok(var.into());
        }
        let c = match self {
            Lit::Int(x) => {
                let value = meta
                    .intern
                    .get(x)
                    .unwrap()
                    .parse()
                    .map_err(|_| Fault::InvalidInt(self.clone()))?;
                Const::Int(value)
            }
            Lit::Float(x) => {
                let value = meta
                    .intern
                    .get(x)
                    .unwrap()
                    .parse::<f64>()
                    .map_err(|_| Fault::InvalidFloat(self.clone()))?
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
                                .ok_or(Fault::InvalidStrChunk(chunk.clone()))?;
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
                                _ => return Err(Fault::InvalidStrChunk(chunk.clone())),
                            };
                            value.push(c)
                        }
                    }
                }
                Const::Str(value.into())
            }
        };
        ctx.consts.insert(self.clone(), c.clone());
        ctx.push(Instruction::Load(var, c));
        Ok(var.into())
    }
}
