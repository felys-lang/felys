use crate::utils::stages::Elysia;
use crate::utils::stdlib::{io, pink};
use crate::Object;

pub fn stdlib() -> impl Iterator<Item=(usize, &'static str, &'static str, Signature)> {
    [].into_iter()
        .chain(io::LIB)
        .chain(pink::LIB)
        .enumerate()
        .map(|(i, x)| (i, x.0, x.1, x.2))
}

pub type Signature = fn(Vec<Object>, elysia: &Elysia, cs: &mut String) -> Object;
