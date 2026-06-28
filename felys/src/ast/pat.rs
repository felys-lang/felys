use crate::ast::*;

#[derive(Clone, Debug)]
pub enum Pat {
    Any,
    Tuple(BufVec<Pat, 2>),
    Ident(usize),
}
