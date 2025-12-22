use crate::ast::BufVec;
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

    pub fn add(&mut self, path: &BufVec<usize, 1>) {
        let mut map = &mut self.tree;
        let mut iter = path.iter().peekable();

        while let Some(ident) = iter.next() {
            if iter.peek().is_none() {
                map.insert(*ident, Node::Name(*ident));
            } else {
                let node = Node::Node(HashMap::new());
                map = match map.entry(*ident).or_insert(node) {
                    Node::Node(next) => next,
                    Node::Name(_) => return,
                };
            }
        }
    }

    pub fn get(&self, path: &BufVec<usize, 1>) -> Result<usize, Fault> {
        let mut map = &self.tree;
        let mut iter = path.iter().peekable();

        while let Some(ident) = iter.next() {
            let node = map.get(ident).ok_or(Fault::InvalidPath)?;
            match (node, iter.peek().is_none()) {
                (Node::Node(next), false) => map = next,
                (Node::Name(x), true) => return Ok(*x),
                _ => return Err(Fault::InvalidPath),
            }
        }
        Err(Fault::InvalidPath)
    }
}
