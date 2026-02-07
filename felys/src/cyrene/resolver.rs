use crate::utils::ast::{AssOp, Block, Expr, Pat, Stmt};
use crate::utils::function::Pointer;
use crate::utils::namespace::Namespace;
use std::collections::{HashMap, HashSet};

pub type Map = HashMap<usize, Option<(Pointer, usize)>>;

#[derive(Default)]
struct Resolver {
    scope: Vec<HashSet<usize>>,
    map: Map,
}

impl Resolver {
    fn stack(&mut self) {
        self.scope.push(HashSet::new());
    }

    fn unstack(&mut self) {
        self.scope.pop();
    }

    fn define(&mut self, id: usize) {
        self.scope.last_mut().unwrap().insert(id);
    }

    fn contains(&self, id: usize) -> bool {
        for set in self.scope.iter().rev() {
            if set.contains(&id) {
                return true;
            }
        }
        false
    }

    fn link(&mut self, id: usize, ptr: Option<(Pointer, usize)>) {
        self.map.insert(id, ptr);
    }
}

impl Block {
    pub fn semantic<'a>(
        &self,
        args: impl Iterator<Item = &'a usize>,
        namespace: &Namespace,
    ) -> Result<Map, &'static str> {
        let mut resolver = Resolver::default();
        resolver.stack();
        for arg in args {
            resolver.define(*arg);
        }
        self.resolve(namespace, &mut resolver)?;
        resolver.unstack();
        Ok(resolver.map)
    }

    fn resolve(&self, namespace: &Namespace, resolver: &mut Resolver) -> Result<(), &'static str> {
        for stmt in self.0.iter() {
            stmt.resolve(namespace, resolver)?;
        }
        Ok(())
    }
}

impl Pat {
    fn resolve(&self, namespace: &Namespace, resolver: &mut Resolver) -> Result<(), &'static str> {
        match self {
            Pat::Any => {}
            Pat::Tuple(tuple) => {
                for pat in tuple.iter() {
                    pat.resolve(namespace, resolver)?;
                }
            }
            Pat::Ident(id) => resolver.define(*id),
        }
        Ok(())
    }

    fn unpack(&self, resolver: &mut Resolver) -> Result<(), &'static str> {
        match self {
            Pat::Any => {}
            Pat::Tuple(tuple) => {
                for pat in tuple.iter() {
                    pat.unpack(resolver)?;
                }
            }
            Pat::Ident(id) => {
                if !resolver.contains(*id) {
                    return Err("variable not defined");
                }
            }
        }
        Ok(())
    }
}

impl Stmt {
    fn resolve(&self, namespace: &Namespace, resolver: &mut Resolver) -> Result<(), &'static str> {
        match self {
            Stmt::Empty => {}
            Stmt::Expr(expr) | Stmt::Semi(expr) => expr.resolve(namespace, resolver)?,
            Stmt::Assign(pat, op, expr) => {
                expr.resolve(namespace, resolver)?;
                if !matches!(op, AssOp::Eq) {
                    pat.unpack(resolver)?
                }
                pat.resolve(namespace, resolver)?;
            }
        }
        Ok(())
    }
}

impl Expr {
    fn resolve(&self, namespace: &Namespace, resolver: &mut Resolver) -> Result<(), &'static str> {
        match self {
            Expr::Block(block) | Expr::Loop(block) => {
                resolver.stack();
                block.resolve(namespace, resolver)?;
                resolver.unstack();
            }
            Expr::Break(expr) => {
                if let Some(expr) = expr {
                    expr.resolve(namespace, resolver)?;
                }
            }
            Expr::Continue => {}
            Expr::For(pat, expr, block) => {
                expr.resolve(namespace, resolver)?;
                resolver.stack();
                pat.resolve(namespace, resolver)?;
                block.resolve(namespace, resolver)?;
                resolver.unstack();
            }
            Expr::If(expr, block, otherwise) => {
                expr.resolve(namespace, resolver)?;
                resolver.stack();
                block.resolve(namespace, resolver)?;
                resolver.unstack();
                if let Some(otherwise) = otherwise {
                    otherwise.resolve(namespace, resolver)?;
                }
            }
            Expr::Return(expr)
            | Expr::Field(expr, _)
            | Expr::Paren(expr)
            | Expr::Unary(_, expr) => expr.resolve(namespace, resolver)?,
            Expr::While(expr, block) => {
                expr.resolve(namespace, resolver)?;
                resolver.stack();
                block.resolve(namespace, resolver)?;
                resolver.unstack();
            }
            Expr::Binary(expr, _, other) | Expr::Index(expr, other) => {
                expr.resolve(namespace, resolver)?;
                other.resolve(namespace, resolver)?;
            }
            Expr::Call(expr, args) | Expr::Method(expr, _, args) => {
                expr.resolve(namespace, resolver)?;
                if let Some(args) = args {
                    for arg in args.iter() {
                        arg.resolve(namespace, resolver)?;
                    }
                }
            }
            Expr::Tuple(args) => {
                for arg in args.iter() {
                    arg.resolve(namespace, resolver)?;
                }
            }
            Expr::List(args) => {
                if let Some(args) = args {
                    for arg in args.iter() {
                        arg.resolve(namespace, resolver)?;
                    }
                }
            }
            Expr::Lit(_) => {}
            Expr::Path(i, path) => {
                if path.len() == 1 && resolver.contains(path.buffer()[0]) {
                    resolver.link(*i, None)
                } else if let Some(ptr) = namespace.get(path.iter()) {
                    resolver.link(*i, Some(ptr))
                } else {
                    return Err("not defined nor is a function");
                }
            }
        }
        Ok(())
    }
}
