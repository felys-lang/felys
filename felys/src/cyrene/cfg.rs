use crate::ast::{Block, Expr, Lit, Path, Stmt};
use crate::cyrene::common::{Context, Function};
use crate::cyrene::{Dst, Namespace};
use crate::error::Fault;

impl Block {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, ns: &Namespace) -> Result<Dst, Fault> {
        let mut iter = self.0.iter().peekable();
        while let Some(stmt) = iter.next() {
            let ret = stmt.ir(f, ctx, ns)?;
            if ret.var().is_ok() {
                return if iter.peek().is_none() {
                    Ok(ret)
                } else {
                    Err(Fault::BlockEarlyEnd)
                };
            }
        }
        Ok(Dst::none())
    }
}

impl Stmt {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, ns: &Namespace) -> Result<Dst, Fault> {
        match self {
            Stmt::Empty => Ok(Dst::none()),
            Stmt::Expr(expr) => expr.ir(f, ctx, ns),
            Stmt::Semi(expr) => expr.ir(f, ctx, ns).and(Ok(Dst::none())),
            Stmt::Assign(_, _, _) => todo!(),
        }
    }
}

impl Expr {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, ns: &Namespace) -> Result<Dst, Fault> {
        match self {
            Expr::Block(block) => block.ir(f, ctx, ns),
            Expr::Break(expr) => {
                let ret = match expr {
                    Some(x) => x.ir(f, ctx, ns)?,
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
                let cond = expr.ir(f, ctx, ns)?.var()?;
                let mut end = ctx.label();
                f.branch(cond, false, end);
                let mut then = block.ir(f, ctx, ns)?;
                let mut void = true;
                if let Some(alt) = alter {
                    let tmp = end;
                    end = ctx.label();
                    f.jump(end);
                    f.append(tmp);
                    let var = alt.ir(f, ctx, ns)?;
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
                block.ir(f, ctx, ns)?;
                let (_, alive) = ctx.writebacks.pop().unwrap();
                ctx.loops.pop();
                f.jump(start);
                f.append(end);
                let dst = if alive { var.into() } else { Dst::none() };
                Ok(dst)
            }
            Expr::Return(expr) => {
                let ret = match expr {
                    Some(x) => x.ir(f, ctx, ns)?,
                    None => Dst::none(),
                };
                f.ret(ret.var().ok());
                Ok(Dst::none())
            }
            Expr::While(expr, block) => {
                let start = ctx.label();
                f.append(start);
                let end = ctx.label();
                let cond = expr.ir(f, ctx, ns)?.var()?;
                f.branch(cond, false, end);
                ctx.loops.push((start, end));
                block.ir(f, ctx, ns)?;
                ctx.loops.pop();
                f.jump(start);
                f.append(end);
                Ok(Dst::none())
            }
            Expr::Binary(lhs, op, rhs) => {
                let l = lhs.ir(f, ctx, ns)?.var()?;
                let r = rhs.ir(f, ctx, ns)?.var()?;
                let var = ctx.var();
                f.binary(var, l, op.clone(), r);
                Ok(var.into())
            }
            Expr::Lambda(_, _) => todo!(),
            Expr::Call(_, _) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::List(_) => todo!(),
            Expr::Lit(lit) => lit.ir(f, ctx),
            Expr::Paren(expr) => expr.ir(f, ctx, ns),
            Expr::Unary(op, inner) => {
                let i = inner.ir(f, ctx, ns)?.var()?;
                let var = ctx.var();
                f.unary(var, op.clone(), i);
                Ok(var.into())
            }
            Expr::Field(_, _) => todo!(),
            Expr::Path(path) => {
                let id = match path {
                    Path::Std(x) => ns.std.get(x)?,
                    Path::Default(x) => ns.default.get(x)?,
                };
                let var = ctx.var();
                f.load(var, id);
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
