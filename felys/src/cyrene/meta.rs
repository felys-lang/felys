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
    pub fields: Box<[usize]>,
    pub methods: HashMap<usize, usize>,
}

impl Group {
    pub fn new(fields: Vec<usize>) -> Self {
        let mut indices = HashMap::new();
        for (i, field) in fields.iter().enumerate() {
            indices.insert(*field, i);
        }
        Self {
            indices,
            fields: fields.into_boxed_slice(),
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

    pub fn add<'a>(&mut self, path: impl Iterator<Item = &'a usize>) -> Option<usize> {
        let mut map = &mut self.tree;
        let mut iter = path.peekable();

        while let Some(ident) = iter.next() {
            if iter.peek().is_none() {
                let id = self.ids;
                if map.insert(*ident, Node::Id(id)).is_some() {
                    return None;
                }
                self.ids += 1;
                return Some(id);
            } else {
                let node = Node::Node(HashMap::new());
                map = match map.entry(*ident).or_insert(node) {
                    Node::Node(next) => next,
                    Node::Id(_) => return None,
                };
            }
        }
        None
    }

    pub fn get<'a>(&self, path: impl Iterator<Item = &'a usize>) -> Option<usize> {
        let mut map = &self.tree;
        let mut iter = path.peekable();

        while let Some(ident) = iter.next() {
            let node = map.get(ident)?;
            match (node, iter.peek().is_none()) {
                (Node::Node(next), false) => map = next,
                (Node::Id(x), true) => return Some(*x),
                _ => return None,
            }
        }
        None
    }
}
