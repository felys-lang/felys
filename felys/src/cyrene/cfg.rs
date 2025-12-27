use crate::ast::{AssOp, BinOp, Block, Bool, Chunk, Expr, Lit, Pat, Stmt};
use crate::cyrene::common::{Context, Function};
use crate::cyrene::{Const, Dst, Instruction, Meta, Var};
use crate::error::Fault;

impl Block {
    pub fn ir(
        &self,
        f: &mut Function,
        ctx: &mut Context,
        meta: &Meta,
        init: Option<(&Pat, Var)>,
    ) -> Result<Dst, Fault> {
        let mut iter = self.0.iter().peekable();
        let mut result = Ok(Dst::none());
        ctx.stack();
        if let Some((pat, var)) = init {
            pat.ir(f, ctx, &None, var)?;
        }
        while let Some(stmt) = iter.next() {
            let ret = stmt.ir(f, ctx, meta)?;
            if ret.var().is_ok() {
                if iter.peek().is_none() {
                    result = Ok(ret);
                } else {
                    result = Err(Fault::BlockEarlyEnd);
                };
                break;
            }
        }
        ctx.unstack();
        result
    }
}

impl Stmt {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, meta: &Meta) -> Result<Dst, Fault> {
        match self {
            Stmt::Empty => Ok(Dst::none()),
            Stmt::Expr(expr) => expr.ir(f, ctx, meta),
            Stmt::Semi(expr) => expr.ir(f, ctx, meta).and(Ok(Dst::none())),
            Stmt::Assign(pat, op, expr) => {
                let op = match op {
                    AssOp::AddEq => Some(BinOp::Add),
                    AssOp::SubEq => Some(BinOp::Sub),
                    AssOp::MulEq => Some(BinOp::Mul),
                    AssOp::DivEq => Some(BinOp::Div),
                    AssOp::ModEq => Some(BinOp::Mod),
                    AssOp::Eq => None,
                };
                let var = expr.ir(f, ctx, meta)?.var()?;
                pat.ir(f, ctx, &op, var)?;
                Ok(Dst::none())
            }
        }
    }
}

impl Pat {
    pub fn ir(
        &self,
        f: &mut Function,
        ctx: &mut Context,
        op: &Option<BinOp>,
        mut var: Var,
    ) -> Result<Dst, Fault> {
        match self {
            Pat::Any => {}
            Pat::Tuple(tup) => {
                for (i, pat) in tup.iter().enumerate() {
                    let dst = ctx.var();
                    f.add(Instruction::Field(dst, var, i));
                    pat.ir(f, ctx, op, dst)?;
                }
            }
            Pat::Ident(id) => {
                if let Some(op) = op {
                    let lhs = ctx.get(*id).ok_or(Fault::InvalidPath)?;
                    let dst = ctx.var();
                    f.add(Instruction::Binary(dst, lhs, op.clone(), var));
                    var = dst;
                }
                ctx.add(*id, var);
            }
        }
        Ok(Dst::none())
    }
}

impl Expr {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, meta: &Meta) -> Result<Dst, Fault> {
        match self {
            Expr::Block(block) => block.ir(f, ctx, meta, None),
            Expr::Break(expr) => {
                let ret = match expr {
                    Some(x) => x.ir(f, ctx, meta)?,
                    None => Dst::none(),
                };
                let (dst, alive) = ctx.writebacks.last_mut().unwrap();
                if *alive {
                    if let Ok(wb) = ret.var() {
                        f.add(Instruction::Copy(*dst, wb))
                    } else {
                        *alive = false;
                    }
                }
                let (_, end) = ctx.loops.last().unwrap();
                f.add(Instruction::Jump(*end));
                Ok(Dst::none())
            }
            Expr::Continue => {
                let (start, _) = ctx.loops.last().unwrap();
                f.add(Instruction::Jump(*start));
                Ok(Dst::none())
            }
            Expr::For(pat, expr, block) => {
                let iterator = expr.ir(f, ctx, meta)?.var()?;
                let len = ctx.var();
                f.add(Instruction::Field(len, iterator, 1));

                let i = ctx.var();
                f.add(Instruction::Load(i, Const::Int(0)));
                let one = ctx.var();
                f.add(Instruction::Load(one, Const::Int(1)));

                let start = ctx.label();
                f.append(start);
                let end = ctx.label();
                let cond = ctx.var();
                f.add(Instruction::Binary(cond, i, BinOp::Le, len));
                f.add(Instruction::Branch(cond, false, end));

                ctx.loops.push((start, end));
                let element = ctx.var();
                f.add(Instruction::Index(element, iterator, i));
                block.ir(f, ctx, meta, Some((pat, element)))?;
                f.add(Instruction::Binary(i, i, BinOp::Add, one));
                ctx.loops.pop();

                f.add(Instruction::Jump(start));
                f.append(end);
                Ok(Dst::none())
            }
            Expr::Index(expr, index) => {
                let val = expr.ir(f, ctx, meta)?.var()?;
                let idx = index.ir(f, ctx, meta)?.var()?;
                let var = ctx.var();
                f.add(Instruction::Index(var, val, idx));
                Ok(var.into())
            }
            Expr::If(expr, block, alter) => {
                let cond = expr.ir(f, ctx, meta)?.var()?;
                let mut end = ctx.label();
                f.add(Instruction::Branch(cond, false, end));
                let mut then = block.ir(f, ctx, meta, None)?;
                let mut void = true;
                if let Some(alt) = alter {
                    let tmp = end;
                    end = ctx.label();
                    f.add(Instruction::Jump(end));
                    f.append(tmp);
                    let var = alt.ir(f, ctx, meta)?;
                    if let Ok(ret) = then.var() {
                        f.add(Instruction::Copy(ret, var.var()?));
                        void = false;
                    }
                }
                f.append(end);
                if void {
                    then = Dst::none();
                }
                Ok(then)
            }
            Expr::Loop(block) => {
                let start = ctx.label();
                f.append(start);
                let end = ctx.label();
                ctx.loops.push((start, end));
                let var = ctx.var();
                ctx.writebacks.push((var, true));
                block.ir(f, ctx, meta, None)?;
                let (_, alive) = ctx.writebacks.pop().unwrap();
                ctx.loops.pop();
                f.add(Instruction::Jump(start));
                f.append(end);
                let dst = if alive { var.into() } else { Dst::none() };
                Ok(dst)
            }
            Expr::Return(expr) => {
                let ret = match expr {
                    Some(x) => x.ir(f, ctx, meta)?,
                    None => Dst::none(),
                };
                f.add(Instruction::Return(ret.var().ok()));
                Ok(Dst::none())
            }
            Expr::While(expr, block) => {
                let start = ctx.label();
                f.append(start);
                let end = ctx.label();
                let cond = expr.ir(f, ctx, meta)?.var()?;
                f.add(Instruction::Branch(cond, false, end));
                ctx.loops.push((start, end));
                block.ir(f, ctx, meta, None)?;
                ctx.loops.pop();
                f.add(Instruction::Jump(start));
                f.append(end);
                Ok(Dst::none())
            }
            Expr::Binary(lhs, op, rhs) => {
                let l = lhs.ir(f, ctx, meta)?.var()?;
                let r = rhs.ir(f, ctx, meta)?.var()?;
                let var = ctx.var();
                f.add(Instruction::Binary(var, l, op.clone(), r));
                Ok(var.into())
            }
            Expr::Call(expr, args) => {
                let func = expr.ir(f, ctx, meta)?.var()?;
                f.add(Instruction::Buffer);
                if let Some(args) = args {
                    for arg in args.iter() {
                        let element = arg.ir(f, ctx, meta)?.var()?;
                        f.add(Instruction::Push(element));
                    }
                }
                let var = ctx.var();
                f.add(Instruction::Call(var, func));
                Ok(var.into())
            }
            Expr::Tuple(args) => {
                f.add(Instruction::Buffer);
                for arg in args.iter() {
                    let element = arg.ir(f, ctx, meta)?.var()?;
                    f.add(Instruction::Push(element));
                }
                let var = ctx.var();
                f.add(Instruction::Tuple(var));
                Ok(var.into())
            }
            Expr::List(args) => {
                f.add(Instruction::Buffer);
                if let Some(args) = args {
                    for arg in args.iter() {
                        let element = arg.ir(f, ctx, meta)?.var()?;
                        f.add(Instruction::Push(element));
                    }
                }
                let var = ctx.var();
                f.add(Instruction::List(var));
                Ok(var.into())
            }
            Expr::Lit(lit) => lit.ir(f, ctx, meta),
            Expr::Paren(expr) => expr.ir(f, ctx, meta),
            Expr::Unary(op, inner) => {
                let i = inner.ir(f, ctx, meta)?.var()?;
                let var = ctx.var();
                f.add(Instruction::Unary(var, op.clone(), i));
                Ok(var.into())
            }
            Expr::Path(path) => {
                if path.len() == 1
                    && let Some(var) = ctx.get(path.buffer()[0])
                {
                    return Ok(var.into());
                }

                let id = meta.ns.get(path)?;
                let var = ctx.var();
                f.add(Instruction::Func(var, id));
                Ok(var.into())
            }
        }
    }
}

impl Lit {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, meta: &Meta) -> Result<Dst, Fault> {
        let var = ctx.var();
        if let Some(c) = ctx.cache.get(self) {
            f.add(Instruction::Load(var, c.clone()));
            return Ok(var.into());
        }
        let c = match self {
            Lit::Int(x) => {
                let value = meta
                    .intern
                    .get(x)
                    .ok_or(Fault::StrNotInterned)?
                    .parse()
                    .map_err(|_| Fault::InvalidConst)?;
                Const::Int(value)
            }
            Lit::Float(x) => {
                let value = meta
                    .intern
                    .get(x)
                    .ok_or(Fault::StrNotInterned)?
                    .parse()
                    .map_err(|_| Fault::InvalidConst)?;
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
                            let s = meta.intern.get(x).ok_or(Fault::InvalidConst)?;
                            value.push_str(s);
                        }
                        Chunk::Unicode(x) => {
                            let hex = meta.intern.get(x).ok_or(Fault::InvalidConst)?;
                            let c = u32::from_str_radix(hex, 16)
                                .ok()
                                .and_then(char::from_u32)
                                .ok_or(Fault::InvalidConst)?;
                            value.push(c)
                        }
                        Chunk::Escape(x) => {
                            let str = meta.intern.get(x).ok_or(Fault::InvalidConst)?;
                            let c = match str {
                                "\'" => '\'',
                                "\"" => '"',
                                "n" => '\n',
                                "t" => '\t',
                                "r" => '\r',
                                "\\" => '\\',
                                _ => return Err(Fault::InvalidConst),
                            };
                            value.push(c)
                        }
                    }
                }
                Const::Str(value.into())
            }
        };
        f.add(Instruction::Load(var, c));
        Ok(var.into())
    }
}
