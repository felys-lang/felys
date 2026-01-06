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
}

#[derive(Debug)]
pub enum Bytecode {
    Field(Reg, Reg, usize),
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
