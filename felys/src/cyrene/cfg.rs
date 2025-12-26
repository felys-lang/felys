use crate::ast::{AssOp, BinOp, Block, Expr, Lit, Pat, Stmt};
use crate::cyrene::common::{Context, Function};
use crate::cyrene::{Dst, Meta, Var};
use crate::error::Fault;

impl Block {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, meta: &Meta) -> Result<Dst, Fault> {
        let mut iter = self.0.iter().peekable();
        let mut result = Ok(Dst::none());
        ctx.stack();
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
                    f.field(dst, var, i);
                    pat.ir(f, ctx, op, dst)?;
                }
            }
            Pat::Ident(id) => {
                if let Some(op) = op {
                    let lhs = ctx.get(*id).ok_or(Fault::InvalidPath)?;
                    let dst = ctx.var();
                    f.binary(dst, lhs, op.clone(), var);
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
            Expr::Block(block) => block.ir(f, ctx, meta),
            Expr::Break(expr) => {
                let ret = match expr {
                    Some(x) => x.ir(f, ctx, meta)?,
                    None => Dst::none(),
                };
                let (dst, alive) = ctx.writebacks.last_mut().unwrap();
                if *alive {
                    if let Ok(wb) = ret.var() {
                        f.copy(*dst, wb);
                    } else {
                        *alive = false;
                    }
                }
                let (_, end) = ctx.loops.last().unwrap();
                f.jump(*end);
                Ok(Dst::none())
            }
            Expr::Continue => {
                let (start, _) = ctx.loops.last().unwrap();
                f.jump(*start);
                Ok(Dst::none())
            }
            Expr::For(_, _, _) => todo!(),
            Expr::If(expr, block, alter) => {
                let cond = expr.ir(f, ctx, meta)?.var()?;
                let mut end = ctx.label();
                f.branch(cond, false, end);
                let mut then = block.ir(f, ctx, meta)?;
                let mut void = true;
                if let Some(alt) = alter {
                    let tmp = end;
                    end = ctx.label();
                    f.jump(end);
                    f.append(tmp);
                    let var = alt.ir(f, ctx, meta)?;
                    if let Ok(ret) = then.var() {
                        f.copy(ret, var.var()?);
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
                block.ir(f, ctx, meta)?;
                let (_, alive) = ctx.writebacks.pop().unwrap();
                ctx.loops.pop();
                f.jump(start);
                f.append(end);
                let dst = if alive { var.into() } else { Dst::none() };
                Ok(dst)
            }
            Expr::Return(expr) => {
                let ret = match expr {
                    Some(x) => x.ir(f, ctx, meta)?,
                    None => Dst::none(),
                };
                f.ret(ret.var().ok());
                Ok(Dst::none())
            }
            Expr::While(expr, block) => {
                let start = ctx.label();
                f.append(start);
                let end = ctx.label();
                let cond = expr.ir(f, ctx, meta)?.var()?;
                f.branch(cond, false, end);
                ctx.loops.push((start, end));
                block.ir(f, ctx, meta)?;
                ctx.loops.pop();
                f.jump(start);
                f.append(end);
                Ok(Dst::none())
            }
            Expr::Binary(lhs, op, rhs) => {
                let l = lhs.ir(f, ctx, meta)?.var()?;
                let r = rhs.ir(f, ctx, meta)?.var()?;
                let var = ctx.var();
                f.binary(var, l, op.clone(), r);
                Ok(var.into())
            }
            Expr::Lambda(_, _) => todo!(),
            Expr::Call(_, _) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::List(_) => todo!(),
            Expr::Lit(lit) => lit.ir(f, ctx),
            Expr::Paren(expr) => expr.ir(f, ctx, meta),
            Expr::Unary(op, inner) => {
                let i = inner.ir(f, ctx, meta)?.var()?;
                let var = ctx.var();
                f.unary(var, op.clone(), i);
                Ok(var.into())
            }
            Expr::Path(path) => {
                if path.len() == 1
                    && let Some(var) = ctx.get(path.buffer()[0])
                {
                    return Ok(var.into());
                }

                let id = meta.namespace.get(path)?;
                let var = ctx.var();
                f.func(var, id);
                Ok(var.into())
            }
        }
    }
}

impl Lit {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context) -> Result<Dst, Fault> {
        match self {
            Lit::Int(id) | Lit::Float(id) => {
                let var = ctx.var();
                f.load(var, *id);
                Ok(var.into())
            }
            Lit::Bool(_) => todo!(),
            Lit::Str(_) => todo!(),
        }
    }
}
