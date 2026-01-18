use crate::utils::ast::{Block, BufVec};

#[derive(Clone, Debug)]
pub enum Item {
    Group(usize, BufVec<usize, 1>),
    Impl(usize, BufVec<Impl, 1>),
    Fn(usize, Option<BufVec<usize, 1>>, Block),
    Main(usize, Block),
}

#[derive(Clone, Debug)]
pub enum Impl {
    Associated(usize, Option<BufVec<usize, 1>>, Block),
    Method(usize, Vec<usize>, Block),
}

#[derive(Clone, Debug)]
pub struct Root(pub BufVec<Item, 1>);
