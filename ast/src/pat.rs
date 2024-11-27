use crate::format::Indenter;
use crate::lit::Lit;
use crate::Symbol;
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
            Pat::Tuple(member) => {
                write!(f, "(")?;
                if let Some(first) = member.first() {
                    first.print(indent, f)?
                }
                for each in member.iter().skip(1) {
                    write!(f, ", ")?;
                    each.print(indent, f)?
                }
                write!(f, ")")
            }
            Pat::Lit(x) => x.print(indent, f),
            Pat::Ident(x) => write!(f, "{}", x),
        }
    }
}

pub type Ident = Symbol;