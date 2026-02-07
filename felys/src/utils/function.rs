use crate::utils::ast::{BinOp, UnaOp};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Function {
    pub vars: usize,
    pub entry: Fragment,
    pub fragments: HashMap<usize, Fragment>,
    pub exit: Fragment,
}

impl Function {
    pub fn var(&mut self) -> Var {
        let id = self.vars;
        self.vars += 1;
        id
    }

    pub fn label(&mut self) -> Label {
        let id = self.fragments.len();
        self.fragments.insert(id, Fragment::default());
        Label::Id(id)
    }

    pub fn get(&self, label: Label) -> Option<&Fragment> {
        match label {
            Label::Entry => Some(&self.entry),
            Label::Id(id) => self.fragments.get(&id),
            Label::Exit => Some(&self.exit),
        }
    }

    pub fn get_mut(&mut self, label: Label) -> Option<&mut Fragment> {
        match label {
            Label::Entry => Some(&mut self.entry),
            Label::Id(id) => self.fragments.get_mut(&id),
            Label::Exit => Some(&mut self.exit),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=(Label, &Fragment)> {
        let fragments = self
            .fragments
            .iter()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &self.exit)])
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(Label, &mut Fragment)> {
        let fragments = self
            .fragments
            .iter_mut()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &mut self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &mut self.exit)])
    }

    pub fn rpo(&self) -> Vec<Label> {
        fn dfs(f: &Function, label: Label, visited: &mut HashSet<Label>, order: &mut Vec<Label>) {
            if !visited.insert(label) {
                return;
            }
            match f.get(label).unwrap().terminator.as_ref().unwrap() {
                Terminator::Branch(_, yes, no) => {
                    dfs(f, *yes, visited, order);
                    dfs(f, *no, visited, order);
                }
                Terminator::Jump(target) => {
                    dfs(f, *target, visited, order);
                }
                Terminator::Return(_) => {}
            }
            order.push(label);
        }
        let mut order = Vec::new();
        let mut visited = HashSet::new();
        dfs(self, Label::Entry, &mut visited, &mut order);
        order.reverse();
        order
    }
}

#[derive(Debug, Default)]
pub struct Fragment {
    pub phis: Vec<Phi>,
    pub predecessors: Vec<Label>,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

#[derive(Debug)]
pub struct Phi {
    pub var: Var,
    pub inputs: Vec<(Label, Var)>,
}

#[derive(Debug)]
pub enum Instruction {
    Arg(Var, usize),
    Field(Var, Var, usize),
    Unpack(Var, Var, usize),
    Pointer(Var, Pointer, usize),
    Load(Var, Const),
    Binary(Var, Var, BinOp, Var),
    Unary(Var, UnaOp, Var),
    Call(Var, Var, Vec<Var>),
    List(Var, Vec<Var>),
    Tuple(Var, Vec<Var>),
    Index(Var, Var, Var),
    Method(Var, Var, usize, Vec<Var>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pointer {
    Group,
    Function,
    Rust,
}

#[derive(Debug)]
pub enum Terminator {
    Branch(Var, Label, Label),
    Jump(Label),
    Return(Var),
}

pub type Var = usize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum Label {
    #[default]
    Entry,
    Id(usize),
    Exit,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Const {
    Int(i32),
    Float(u32),
    Bool(bool),
    Str(Rc<str>),
}
