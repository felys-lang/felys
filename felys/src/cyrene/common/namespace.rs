use crate::ast::{BufVec, Ident};
use crate::cyrene::Var;
use crate::error::Fault;
use std::collections::HashMap;

pub struct Namespace {
    pub std: Router,
    pub default: Router,
}

pub struct Router {
    ids: usize,
    tree: HashMap<usize, Node>,
}

enum Node {
    Name(usize),
    Node(HashMap<usize, Node>),
}

impl Router {
    pub fn new() -> Self {
        Self {
            ids: 0,
            tree: HashMap::new(),
        }
    }

    pub fn add(&mut self, path: &BufVec<Ident, 1>) {
        let mut map = &mut self.tree;
        let mut iter = path.iter().peekable();

        while let Some(ident) = iter.next() {
            if iter.peek().is_none() {
                map.insert(ident.0, Node::Name(ident.0));
            } else {
                let node = Node::Node(HashMap::new());
                map = match map.entry(ident.0).or_insert(node) {
                    Node::Node(next) => next,
                    Node::Name(_) => return,
                };
            }
        }
    }

    pub fn get(&self, path: &BufVec<Ident, 1>) -> Result<Var, Fault> {
        let mut map = &self.tree;
        let mut iter = path.iter().peekable();

        while let Some(ident) = iter.next() {
            let node = map.get(&ident.0).ok_or(Fault::InvalidPath)?;
            match (node, iter.peek().is_none()) {
                (Node::Node(next), false) => map = next,
                (Node::Name(x), true) => return Ok(*x),
                _ => return Err(Fault::InvalidPath),
            }
        }
        Err(Fault::InvalidPath)
    }
}
