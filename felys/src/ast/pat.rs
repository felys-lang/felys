use crate::ast::format::Indenter;
use crate::ast::lit::Lit;
use crate::ast::Id;
use std::fmt::Formatter;

#[derive(Clone, Debug)]
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

impl Indenter for Pat {
    fn print(&self, indent: usize, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pat::Any => write!(f, "_"),
            Pat::Tuple(tup) => {
                write!(f, "(")?;
                if let Some(first) = tup.first() {
                    first.print(indent, f)?
                }
                for each in tup.iter().skip(1) {
                    write!(f, ", ")?;
                    each.print(indent, f)?
                }
                write!(f, ")")
            }
            Pat::Lit(lit) => lit.print(indent, f),
            Pat::Ident(ident) => write!(f, "{}", ident),
        }
    }
}

pub type Ident = Id;