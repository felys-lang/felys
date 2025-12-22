use crate::ast::*;

#[derive(Clone, Debug)]
pub enum Pat {
    Any,
    Lit(Lit),
    Tuple(BufVec<Pat, 2>),
    Group(BufVec<Pat, 1>),
    Ident(Ident),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Ident(pub usize);
