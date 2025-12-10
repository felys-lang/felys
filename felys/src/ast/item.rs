use crate::ast::{Block, BufVec, Ident};

#[derive(Clone, Debug)]
pub enum Item {
    Group(Ident, BufVec<Ident, 1>),
    Impl(Ident, BufVec<Impl, 1>),
    Fn(Ident, Option<BufVec<Ident, 1>>, Block),
    Main(Ident, Block),
}

#[derive(Clone, Debug)]
pub enum Impl {
    Associated(Ident, Option<BufVec<Ident, 1>>, Block),
    Method(Ident, Vec<Ident>, Block),
}

#[derive(Clone, Debug)]
pub struct Root(pub BufVec<Item, 1>);
