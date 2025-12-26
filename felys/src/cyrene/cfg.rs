use crate::ast::{AssOp, BinOp, Block, Expr, Lit, Pat, Stmt};
use crate::cyrene::common::{Context, Function};
use crate::cyrene::{Dst, Instruction, Namespace, Var};
use crate::error::Fault;

impl Block {
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, ns: &Namespace) -> Result<Dst, Fault> {
        let mut iter = self.0.iter().peekable();
        let mut result = Ok(Dst::none());
        ctx.stack();
        while let Some(stmt) = iter.next() {
            let ret = stmt.ir(f, ctx, ns)?;
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
    pub fn ir(&self, f: &mut Function, ctx: &mut Context, ns: &Namespace) -> Result<Dst, Fault> {
        match self {
            Stmt::Empty => Ok(Dst::none()),
            Stmt::Expr(expr) => expr.ir(f, ctx, ns),
            Stmt::Semi(expr) => expr.ir(f, ctx, ns).and(Ok(Dst::none())),
            Stmt::Assign(pat, op, expr) => {
                let op = match op {
                    AssOp::AddEq => Some(BinOp::Add),
                    AssOp::SubEq => Some(BinOp::Sub),
                    AssOp::MulEq => Some(BinOp::Mul),
                    AssOp::DivEq => Some(BinOp::Div),
                    AssOp::ModEq => Some(BinOp::Mod),
                    AssOp::Eq => None,
                };
                let var = expr.ir(f, ctx, ns)?.var()?;
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
            Expr::For(p, _, _) => todo!(),
            Expr::If(expr, block, alter) => {
                let cond = expr.ir(f, ctx, ns)?.var()?;
                let mut end = ctx.label();
                f.add(Instruction::Branch(cond, false, end));
                let mut then = block.ir(f, ctx, ns)?;
                let mut void = true;
                if let Some(alt) = alter {
                    let tmp = end;
                    end = ctx.label();
                    f.add(Instruction::Jump(end));
                    f.append(tmp);
                    let var = alt.ir(f, ctx, ns)?;
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
                block.ir(f, ctx, ns)?;
                let (_, alive) = ctx.writebacks.pop().unwrap();
                ctx.loops.pop();
                f.add(Instruction::Jump(start));
                f.append(end);
                let dst = if alive { var.into() } else { Dst::none() };
                Ok(dst)
            }
            Expr::Return(expr) => {
                let ret = match expr {
                    Some(x) => x.ir(f, ctx, ns)?,
                    None => Dst::none(),
                };
                f.add(Instruction::Return(ret.var().ok()));
                Ok(Dst::none())
            }
            Expr::While(expr, block) => {
                let start = ctx.label();
                f.append(start);
                let end = ctx.label();
                let cond = expr.ir(f, ctx, ns)?.var()?;
                f.add(Instruction::Branch(cond, false, end));
                ctx.loops.push((start, end));
                block.ir(f, ctx, ns)?;
                ctx.loops.pop();
                f.add(Instruction::Jump(start));
                f.append(end);
                Ok(Dst::none())
            }
            Expr::Binary(lhs, op, rhs) => {
                let l = lhs.ir(f, ctx, ns)?.var()?;
                let r = rhs.ir(f, ctx, ns)?.var()?;
                let var = ctx.var();
                f.add(Instruction::Binary(var, l, op.clone(), r));
                Ok(var.into())
            }
            Expr::Call(expr, args) => {
                f.add(Instruction::Buffer);
                if let Some(args) = args {
                    for arg in args.iter() {
                        let element = arg.ir(f, ctx, ns)?.var()?;
                        f.add(Instruction::Push(element));
                    }
                }
                let func = expr.ir(f, ctx, ns)?.var()?;
                let var = ctx.var();
                f.add(Instruction::Call(var, func));
                Ok(var.into())
            }
            Expr::Tuple(args) => {
                f.add(Instruction::Buffer);
                for arg in args.iter() {
                    let element = arg.ir(f, ctx, ns)?.var()?;
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
                        let element = arg.ir(f, ctx, ns)?.var()?;
                        f.add(Instruction::Push(element));
                    }
                }
                let var = ctx.var();
                f.add(Instruction::List(var));
                Ok(var.into())
            }
            Expr::Lit(lit) => lit.ir(f, ctx),
            Expr::Paren(expr) => expr.ir(f, ctx, ns),
            Expr::Unary(op, inner) => {
                let i = inner.ir(f, ctx, ns)?.var()?;
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

                let id = ns.get(path)?;
                let var = ctx.var();
                f.add(Instruction::Func(var, id));
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
                f.add(Instruction::Load(var, *id));
                Ok(var.into())
            }
            Lit::Bool(_) => todo!(),
            Lit::Str(_) => todo!(),
        }
    }
}
