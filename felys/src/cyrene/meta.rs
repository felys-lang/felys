use crate::error::Fault;
use crate::philia093::Intern;
use std::collections::HashMap;

pub struct Meta {
    pub ns: Namespace,
    pub constructors: Namespace,
    pub intern: Intern,
    pub groups: HashMap<usize, Group>,
}

#[derive(Debug)]
pub struct Group {
    pub indices: HashMap<usize, usize>,
    pub methods: HashMap<usize, usize>,
}

impl Group {
    pub fn new<'a>(fields: impl Iterator<Item=&'a usize>) -> Self {
        let mut indices = HashMap::new();
        for (i, field) in fields.enumerate() {
            indices.insert(*field, i);
        }
        Self {
            indices,
            methods: HashMap::new(),
        }
    }
}

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

    pub fn add<'a>(&mut self, path: impl Iterator<Item=&'a usize>) -> Result<usize, Fault> {
        let mut map = &mut self.tree;
        let mut iter = path.peekable();

        while let Some(ident) = iter.next() {
            if iter.peek().is_none() {
                let id = self.ids;
                if map.insert(*ident, Node::Id(id)).is_some() {
                    return Err(Fault::InvalidPath);
                }
                self.ids += 1;
                return Ok(id);
            } else {
                let node = Node::Node(HashMap::new());
                map = match map.entry(*ident).or_insert(node) {
                    Node::Node(next) => next,
                    Node::Id(_) => return Err(Fault::InvalidPath),
                };
            }
        }
        Err(Fault::InvalidPath)
    }

    pub fn get<'a>(&self, path: impl Iterator<Item=&'a usize>) -> Result<usize, Fault> {
        let mut map = &self.tree;
        let mut iter = path.peekable();

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
