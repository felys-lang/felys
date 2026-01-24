use crate::philia093::Intern;
use crate::utils::function::Function;
use crate::utils::group::Group;
use crate::utils::ir::Pointer;
use crate::utils::stdlib::utils::stdlib;
use std::collections::HashMap;

pub struct Meta {
    pub namespace: Namespace,
    pub intern: Intern,
    pub groups: HashMap<usize, Group>,
    pub functions: HashMap<usize, Function>,
    pub main: Option<Function>,
}

impl Meta {
    pub fn new(mut intern: Intern) -> Self {
        Self {
            namespace: Namespace::init(&mut intern),
            intern,
            groups: Default::default(),
            functions: Default::default(),
            main: None,
        }
    }
}

pub struct Namespace {
    ids: usize,
    tree: HashMap<usize, Node>,
}

enum Node {
    Group(usize, HashMap<usize, usize>),
    Function(usize),
    Rust(usize),
    Redirect(HashMap<usize, Node>),
}

impl Namespace {
    fn init(intern: &mut Intern) -> Self {
        let mut base = HashMap::new();
        for (i, (sub, inner, _)) in stdlib().enumerate() {
            if let Node::Redirect(x) = base
                .entry(intern.id(sub))
                .or_insert(Node::Redirect(HashMap::new()))
            {
                x.insert(intern.id(inner), Node::Rust(i));
            }
        }

        Self {
            ids: 0,
            tree: HashMap::from([(intern.id("std"), Node::Redirect(base))]),
        }
    }

    pub fn allocate(&mut self, path: &[usize], name: usize) -> Option<usize> {
        let id = self.id();
        let mut cursor = &mut self.tree;

        for space in path {
            let node = cursor
                .entry(*space)
                .or_insert(Node::Redirect(HashMap::new()));
            let Node::Redirect(next) = node else {
                return None;
            };
            cursor = next;
        }
        if cursor
            .insert(name, Node::Group(id, HashMap::new()))
            .is_some()
        {
            return None;
        };
        Some(id)
    }

    pub fn attach(&mut self, path: &[usize], name: usize) -> Option<usize> {
        let id = self.id();
        let mut cursor = &mut self.tree;

        for space in path {
            let node = cursor
                .entry(*space)
                .or_insert(Node::Redirect(HashMap::new()));
            match node {
                Node::Group(_, group) => {
                    return if group.insert(name, id).is_none() {
                        Some(id)
                    } else {
                        None
                    };
                }
                Node::Function(_) | Node::Rust(_) => return None,
                Node::Redirect(next) => cursor = next,
            };
        }
        if cursor.insert(name, Node::Function(id)).is_none() {
            Some(id)
        } else {
            None
        }
    }

    pub fn get<'a>(&self, mut path: impl Iterator<Item = &'a usize>) -> Option<(Pointer, usize)> {
        let mut cursor = &self.tree;
        let mut tmp = None;

        while let Some(space) = path.next() {
            if tmp.is_some() {
                return None;
            }
            match cursor.get(space)? {
                Node::Group(x, methods) => {
                    if let Some(next) = path.next() {
                        tmp = Some((Pointer::Function, *methods.get(next)?));
                    } else {
                        tmp = Some((Pointer::Group, *x));
                    }
                }
                Node::Function(x) => tmp = Some((Pointer::Function, *x)),
                Node::Rust(x) => tmp = Some((Pointer::Rust, *x)),
                Node::Redirect(next) => cursor = next,
            };
        }
        tmp
    }

    fn id(&mut self) -> usize {
        let id = self.ids;
        self.ids += 1;
        id
    }
}
