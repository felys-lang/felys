use crate::acheron::{BinOp, UnaOp};
use crate::cyrene::Group;
use crate::utils::function::{Fragment, Function};
use crate::utils::ir::{Label, Terminator};
use std::collections::{HashMap, HashSet};

pub struct Demiurge {
    pub groups: HashMap<usize, Group>,
    pub fns: HashMap<usize, Function>,
    pub main: Function,
}

impl Function {
    pub fn safe(&self) -> impl Iterator<Item = (Label, &Fragment)> {
        let fragments = self
            .fragments
            .iter()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &self.exit)])
    }

    pub fn cautious(&mut self) -> impl Iterator<Item = (Label, &mut Fragment)> {
        let fragments = self
            .fragments
            .iter_mut()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &mut self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &mut self.exit)])
    }

    pub fn order(&self) -> impl Iterator<Item = Label> {
        let mut order = self.fragments.keys().cloned().collect::<Vec<_>>();
        order.sort_unstable();
        [Label::Entry]
            .into_iter()
            .chain(order.into_iter().map(Label::Id))
            .chain([Label::Exit])
    }

    pub fn rpo(&self) -> Vec<Label> {
        let mut order = Vec::new();
        let mut visited = HashSet::new();
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
        dfs(self, Label::Entry, &mut visited, &mut order);
        order.reverse();
        order
    }
}

#[derive(Debug)]
pub enum Bytecode {
    Field(Reg, Reg, usize),
    Unpack(Reg, Reg, usize),
    Group(Reg, Idx),
    Function(Reg, Idx),
    Load(Reg, Idx),
    Binary(Reg, Reg, BinOp, Reg),
    Unary(Reg, UnaOp, Reg),
    Call(Reg, Reg, Vec<Reg>),
    List(Reg, Vec<Reg>),
    Tuple(Reg, Vec<Reg>),
    Index(Reg, Reg, Reg),
    Method(Reg, Reg, usize, Vec<Reg>),
    Branch(Reg, Idx, Idx),
    Jump(Idx),
    Return(Reg),
    Copy(Reg, Reg),
}

pub type Reg = usize;

pub type Idx = usize;
