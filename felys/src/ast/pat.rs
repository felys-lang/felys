use crate::ast::*;

#[derive(Clone, Debug)]
pub enum Pat {
    /// don't care: `_`
    Any,
    /// literal: `11.11`
    Lit(Lit),
    /// unwrap a group: `(elysia, 11.11)`
    Tuple(BufVec<Pat, 2>),
    /// identifier
    Ident(Ident),
}

#[derive(Clone, Debug)]
pub struct Ident(pub Id);

pub type Id = usize;
