use crate::cyrene::cfg::context::{Context, Id};
use crate::cyrene::resolver::Map;
use crate::philia093::Intern;
use crate::utils::ast::{AssOp, BinOp, Block, Bool, Chunk, Expr, Lit, Pat, Stmt};
use crate::utils::function::{Const, Function, Instruction, Label, Var};

type Stack = Vec<(Label, Label, Option<(Id, Option<bool>)>)>;

impl Block {
    pub fn function(
        &self,
        map: &Map,
        intern: &Intern,
        args: Vec<usize>,
    ) -> Result<Function, &'static str> {
        let mut stk = Vec::new();
        let mut ctx = Context::default();
        ctx.seal(Label::Entry);
        for (i, id) in args.iter().enumerate() {
            let var = ctx.var();
            ctx.push(Instruction::Arg(var, i));
            ctx.define(ctx.cursor, Id::Interned(*id), var);
        }

        if let Some(var) = self.ir(map, intern, &mut ctx, &mut stk)? {
            ctx.define(ctx.cursor, Id::Ret, var);
        }
        ctx.jump(Label::Exit);
        ctx.seal(Label::Exit);

        ctx.cursor = Label::Exit;
        let var = ctx.lookup(ctx.cursor, Id::Ret).ok_or("no return value")?;
        ctx.ret(var);
        Ok(ctx.into())
    }

    fn ir(
        &self,
        map: &Map,
        intern: &Intern,
        ctx: &mut Context,
        stk: &mut Stack,
    ) -> Result<Option<Var>, &'static str> {
        let mut iter = self.0.iter().peekable();
        let mut result = Ok(None);
        while let Some(stmt) = iter.next() {
            let ret = stmt.ir(map, intern, ctx, stk)?;
            if ret.is_some() {
                if iter.peek().is_none() {
                    result = Ok(ret);
                } else {
                    result = Err("block early return");
                };
                break;
            }
        }
        result
    }
}

impl Stmt {
    fn ir(
        &self,
        map: &Map,
        intern: &Intern,
        ctx: &mut Context,
        stk: &mut Stack,
    ) -> Result<Option<Var>, &'static str> {
        match self {
            Stmt::Empty => Ok(None),
            Stmt::Expr(expr) => expr.ir(map, intern, ctx, stk),
            Stmt::Semi(expr) => expr.ir(map, intern, ctx, stk).and(Ok(None)),
            Stmt::Assign(pat, op, expr) => {
                let op = match op {
                    AssOp::AddEq => Some(BinOp::Add),
                    AssOp::SubEq => Some(BinOp::Sub),
                    AssOp::MulEq => Some(BinOp::Mul),
                    AssOp::DivEq => Some(BinOp::Div),
                    AssOp::ModEq => Some(BinOp::Mod),
                    AssOp::Eq => None,
                };
                let var = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                pat.ir(ctx, &op, var)?;
                Ok(None)
            }
        }
    }
}

impl Pat {
    fn ir(&self, ctx: &mut Context, op: &Option<BinOp>, mut rhs: Var) -> Result<(), &'static str> {
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
    fn ir(
        &self,
        map: &Map,
        intern: &Intern,
        ctx: &mut Context,
        stk: &mut Stack,
    ) -> Result<Option<Var>, &'static str> {
        match self {
            Expr::Block(block) => block.ir(map, intern, ctx, stk),
            Expr::Break(expr) => {
                let (_, end, wb) = stk.last().cloned().ok_or("outside loop")?;
                if let Some((id, action)) = wb
                    && action.unwrap_or(true)
                {
                    if let Some(x) = expr
                        && let Some(var) = x.ir(map, intern, ctx, stk)?
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
                let (start, _, _) = stk.last().ok_or("outside loop")?;
                ctx.jump(*start);
                Ok(None)
            }
            Expr::For(pat, expr, block) => {
                let header = ctx.label();
                let body = ctx.label();
                let end = ctx.label();

                let iterable = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;

                let length = ctx.var();
                ctx.push(Instruction::Unpack(length, iterable, 0));

                let i = {
                    let var = ctx.var();
                    ctx.push(Instruction::Load(var, Const::Int(0)));
                    let id = ctx.id();
                    ctx.define(ctx.cursor, id, var);
                    id
                };

                let one = ctx.var();
                ctx.push(Instruction::Load(one, Const::Int(1)));

                ctx.jump(header);

                ctx.cursor = header;
                let cond = ctx.var();
                let instruction = Instruction::Binary(
                    cond,
                    ctx.lookup(ctx.cursor, i).unwrap(),
                    BinOp::Lt,
                    length,
                );
                ctx.push(instruction);
                ctx.branch(cond, body, end);
                ctx.seal(body);

                ctx.cursor = body;
                stk.push((header, end, None));

                let element = ctx.var();
                let index = ctx.lookup(ctx.cursor, i).unwrap();
                let instruction = Instruction::Index(element, iterable, index);
                ctx.push(instruction);

                let var = ctx.var();
                ctx.push(Instruction::Binary(var, index, BinOp::Add, one));
                ctx.define(ctx.cursor, i, var);

                pat.ir(ctx, &None, element)?;
                block.ir(map, intern, ctx, stk)?;

                stk.pop();
                ctx.jump(header);
                ctx.seal(header);
                ctx.seal(end);

                ctx.cursor = end;
                Ok(None)
            }
            Expr::If(expr, block, alter) => {
                let then = ctx.label();
                let otherwise = ctx.label();
                let join = ctx.label();
                let ret = ctx.id();

                let cond = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                ctx.branch(cond, then, otherwise);
                ctx.seal(then);
                ctx.seal(otherwise);

                let mut returned = [false, false];
                let mut joined = [false, false];

                ctx.cursor = then;
                if let Some(var) = block.ir(map, intern, ctx, stk)? {
                    ctx.define(ctx.cursor, ret, var);
                    returned[0] = true;
                }
                joined[0] = ctx.jump(join);

                ctx.cursor = otherwise;
                if let Some(alt) = alter
                    && let Some(var) = alt.ir(map, intern, ctx, stk)?
                {
                    ctx.define(ctx.cursor, ret, var);
                    returned[1] = true;
                }
                joined[1] = ctx.jump(join);

                ctx.seal(join);

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
                block.ir(map, intern, ctx, stk)?;
                let action = stk.pop().unwrap().2.unwrap().1;

                ctx.jump(body);
                ctx.seal(body);
                ctx.seal(end);

                ctx.cursor = end;
                if action.unwrap_or(false) {
                    let var = ctx.lookup(ctx.cursor, ret).unwrap();
                    Ok(Some(var))
                } else {
                    Ok(None)
                }
            }
            Expr::Return(expr) => {
                let var = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
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
                let cond = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                ctx.branch(cond, body, end);
                ctx.seal(body);

                ctx.cursor = body;
                stk.push((header, end, None));
                block.ir(map, intern, ctx, stk)?;
                stk.pop();
                ctx.jump(header);
                ctx.seal(header);
                ctx.seal(end);

                ctx.cursor = end;
                Ok(None)
            }
            Expr::Binary(lhs, op, rhs) => {
                let l = lhs.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                let r = rhs.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                let var = ctx.var();
                ctx.push(Instruction::Binary(var, l, *op, r));
                Ok(var.into())
            }
            Expr::Call(expr, args) => {
                let callable = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                let mut params = Vec::new();
                if let Some(args) = args {
                    for arg in args.iter() {
                        let param = arg.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                        params.push(param);
                    }
                }
                let var = ctx.var();
                ctx.push(Instruction::Call(var, callable, params));
                Ok(var.into())
            }
            Expr::Field(expr, id) => {
                let src = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                let var = ctx.var();
                ctx.push(Instruction::Field(var, src, *id));
                Ok(var.into())
            }
            Expr::Method(expr, id, args) => {
                let src = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                let mut params = Vec::new();
                if let Some(args) = args {
                    for arg in args.iter() {
                        let param = arg.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                        params.push(param);
                    }
                }
                let var = ctx.var();
                ctx.push(Instruction::Method(var, src, *id, params));
                Ok(var.into())
            }
            Expr::Index(expr, index) => {
                let src = expr.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                let idx = index.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                let var = ctx.var();
                ctx.push(Instruction::Index(var, src, idx));
                Ok(var.into())
            }
            Expr::Tuple(args) => {
                let mut params = Vec::new();
                for arg in args.iter() {
                    let param = arg.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
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
                        let param = arg.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                        params.push(param);
                    }
                }
                let var = ctx.var();
                ctx.push(Instruction::List(var, params));
                Ok(var.into())
            }
            Expr::Lit(lit) => lit.ir(intern, ctx),
            Expr::Paren(expr) => expr.ir(map, intern, ctx, stk),
            Expr::Unary(op, inner) => {
                let i = inner.ir(map, intern, ctx, stk)?.ok_or("no return value")?;
                let var = ctx.var();
                ctx.push(Instruction::Unary(var, *op, i));
                Ok(var.into())
            }
            Expr::Path(i, path) => {
                let var = if let Some((pt, ptr)) = map.get(i).unwrap() {
                    let var = ctx.var();
                    ctx.push(Instruction::Pointer(var, *pt, *ptr));
                    var
                } else {
                    let id = Id::Interned(path.buffer()[0]);
                    ctx.lookup(ctx.cursor, id).unwrap()
                };
                Ok(var.into())
            }
        }
    }
}

impl Lit {
    fn ir(&self, intern: &Intern, ctx: &mut Context) -> Result<Option<Var>, &'static str> {
        let var = ctx.var();
        if let Some(c) = ctx.consts.get(self) {
            ctx.push(Instruction::Load(var, c.clone()));
            return Ok(var.into());
        }
        let c = match self {
            Lit::Int(x) => {
                let value = intern.get(x).unwrap().parse().map_err(|_| "invalid int")?;
                Const::Int(value)
            }
            Lit::Float(x) => {
                let value = intern
                    .get(x)
                    .unwrap()
                    .parse::<f32>()
                    .map_err(|_| "invalid float")?
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
                            let s = intern.get(x).unwrap();
                            value.push_str(s);
                        }
                        Chunk::Unicode(x) => {
                            let hex = intern.get(x).unwrap();
                            let c = u32::from_str_radix(hex, 16)
                                .ok()
                                .and_then(char::from_u32)
                                .ok_or("invalid str chunk")?;
                            value.push(c)
                        }
                        Chunk::Escape(x) => {
                            let str = intern.get(x).unwrap();
                            let c = match str {
                                "\'" => '\'',
                                "\"" => '"',
                                "n" => '\n',
                                "t" => '\t',
                                "r" => '\r',
                                "\\" => '\\',
                                _ => return Err("invalid str chunk"),
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
