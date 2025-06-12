use crate::ast::*;

#[derive(Clone, Debug)]
pub enum Pat {
    /// don't care: `_`
    Any,
    /// literal: `11.11`
    Lit(Lit),
    /// unwrap a group: `(elysia, 11.11)`
    Tuple(Vec<Pat>),
    /// identifier
    Ident(Ident),
}

pub type Ident = usize;
