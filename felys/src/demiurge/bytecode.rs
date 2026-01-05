use crate::ast::{BinOp, UnaOp};
use crate::cyrene::{Fragment, Function, Group, Label};
use crate::philia093::Intern;
use std::collections::HashMap;

pub struct Demiurge {
    pub groups: HashMap<usize, Group>,
    pub fns: HashMap<usize, Function>,
    pub main: Function,
    pub intern: Intern,
}

impl Function {
    pub fn safe(&self) -> impl Iterator<Item=(Label, &Fragment)> {
        let fragments = self
            .fragments
            .iter()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &self.exit)])
    }

    pub fn dangerous(&mut self) -> impl Iterator<Item=(Label, &mut Fragment)> {
        let fragments = self
            .fragments
            .iter_mut()
            .map(|(id, frag)| (Label::Id(*id), frag));
        [(Label::Entry, &mut self.entry)]
            .into_iter()
            .chain(fragments)
            .chain([(Label::Exit, &mut self.exit)])
    }

    pub fn get(&self, label: Label) -> Option<&Fragment> {
        match label {
            Label::Entry => Some(&self.entry),
            Label::Id(id) => self.fragments.get(&id),
            Label::Exit => Some(&self.exit),
        }
    }

    pub fn modify(&mut self, label: Label) -> Option<&mut Fragment> {
        match label {
            Label::Entry => Some(&mut self.entry),
            Label::Id(id) => self.fragments.get_mut(&id),
            Label::Exit => Some(&mut self.exit),
        }
    }
}

#[derive(Debug)]
pub enum Bytecode {
    Field(Reg, Reg, usize),
    Func(Reg, Idx),
    Load(Reg, Idx),
    Binary(Reg, Reg, BinOp, Reg),
    Unary(Reg, UnaOp, Reg),
    Copy(Reg, Reg),
    Branch(Reg, bool, Idx),
    Jump(Idx),
    Return(Option<Reg>),
    Buffer,
    Push(Reg),
    Call(Reg, Reg),
    List(Reg),
    Tuple(Reg),
    Index(Reg, Reg, Reg),
    Method(Reg, Reg, usize),
    Group(Reg, Idx),
}

pub type Reg = usize;

pub type Idx = usize;
