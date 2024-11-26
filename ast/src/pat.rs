use crate::lit::Lit;
use crate::Symbol;

pub enum Pat {
    /// don't care: `_`
    Any,
    /// unwrap a group: `(elysia, 11.11)`
    Tuple(Vec<Pat>),
    /// literals: `"elysia"`, `11.11`, `true`
    Lit(Lit),
    /// identifier
    Ident(Ident),
}

pub type Ident = Symbol;