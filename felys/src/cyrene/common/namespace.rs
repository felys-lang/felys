use crate::ast::BufVec;
use crate::error::Fault;
use std::collections::HashMap;

pub struct Namespace {
    ids: usize,
    tree: HashMap<usize, Node>,
}

enum Node {
    Id(usize),
    Node(HashMap<usize, Node>),
}

impl Namespace {
    pub fn new() -> Self {
        Self {
            ids: 0,
            tree: HashMap::new(),
        }
    }

    pub fn add(&mut self, path: &BufVec<usize, 1>) -> Result<(), Fault> {
        let mut map = &mut self.tree;
        let mut iter = path.iter().peekable();

        while let Some(ident) = iter.next() {
            if iter.peek().is_none() {
                if map.insert(*ident, Node::Id(self.ids)).is_some() {
                    return Err(Fault::InvalidPath);
                }
                self.ids += 1;
            } else {
                let node = Node::Node(HashMap::new());
                map = match map.entry(*ident).or_insert(node) {
                    Node::Node(next) => next,
                    Node::Id(_) => return Err(Fault::InvalidPath),
                };
            }
        }
        Ok(())
    }

    pub fn get(&self, path: &BufVec<usize, 1>) -> Result<usize, Fault> {
        let mut map = &self.tree;
        let mut iter = path.iter().peekable();

        while let Some(ident) = iter.next() {
            let node = map.get(ident).ok_or(Fault::InvalidPath)?;
            match (node, iter.peek().is_none()) {
                (Node::Node(next), false) => map = next,
                (Node::Id(x), true) => return Ok(*x),
                _ => return Err(Fault::InvalidPath),
            }
        }
        Err(Fault::InvalidPath)
    }
}
