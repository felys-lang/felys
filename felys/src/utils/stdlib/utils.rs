use crate::elysia::Elysia;
use crate::utils::stdlib::{io, pink};
use crate::Object;

pub fn stdlib() -> impl Iterator<Item=(&'static str, &'static str, Signature)> {
    [].into_iter().chain(io::LIB).chain(pink::LIB)
}

pub type Signature = fn(Vec<Object>, elysia: &Elysia) -> Object;
