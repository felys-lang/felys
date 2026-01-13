use crate::acheron::*;

#[derive(Clone, Debug)]
pub enum Pat {
    Any,
    Tuple(BufVec<Pat, 2>),
    Ident(usize),
}
