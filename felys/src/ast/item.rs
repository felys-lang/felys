use crate::ast::{Block, BufVec};

#[derive(Clone, Debug)]
pub enum Item {
    Fn(usize, Option<BufVec<usize, 1>>, Block),
    Main(usize, Block),
}

#[derive(Clone, Debug)]
pub struct Root(pub BufVec<Item, 1>);
