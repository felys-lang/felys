use crate::ast::{BinOp, Lit, UnaOp};
use crate::error::Fault;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::panic::Location;
use std::rc::Rc;

#[derive(Default)]
pub struct Context {
    ids: usize,
    pub cursor: Label,
    pub cache: HashMap<Lit, Const>,

    defs: HashMap<Label, HashMap<Id, Var>>,
    incompleted: HashMap<Label, HashMap<Id, Var>>,
    sealed: HashSet<Label>,

    vars: usize,
    labels: usize,
    entry: Fragment,
    fragments: HashMap<usize, Fragment>,
    exit: Fragment,
}

#[derive(Debug)]
pub struct Function {
    pub args: Vec<usize>,
    pub vars: usize,
    pub labels: usize,
    pub entry: Fragment,
    pub fragments: HashMap<usize, Fragment>,
    pub exit: Fragment,
}

impl Function {
    pub fn label(&mut self) -> Label {
        let label = self.labels;
        self.labels += 1;
        Label::Id(label)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Id {
    Interned(usize),
    Tmp(usize),
    Ret,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Const {
    Int(isize),
    Float(u64),
    Bool(bool),
    Str(Rc<str>),
}

impl Context {
    pub fn export(mut self) -> Function {
        let args = self
            .defs
            .remove(&Label::Entry)
            .map_or_else(Vec::new, |map| map.into_values().collect());
        Function {
            args,
            vars: self.vars,
            labels: self.labels,
            entry: self.entry,
            fragments: self.fragments,
            exit: self.exit,
        }
    }

    pub fn id(&mut self) -> Id {
        let id = self.ids;
        self.ids += 1;
        Id::Tmp(id)
    }

    pub fn var(&mut self) -> Var {
        let var = self.vars;
        self.vars += 1;
        var
    }

    pub fn label(&mut self) -> Label {
        let label = self.labels;
        self.labels += 1;
        Label::Id(label)
    }

    pub fn unreachable(&mut self) -> Result<Dst, Fault> {
        let dead = self.label();
        self.add(dead);
        self.cursor = dead;
        Ok(Dst::void())
    }

    pub fn add(&mut self, label: Label) {
        let Label::Id(id) = label else { return };
        self.fragments.insert(id, Fragment::default());
    }

    pub fn get(&mut self, label: Label) -> &mut Fragment {
        match label {
            Label::Entry => &mut self.entry,
            Label::Id(id) => self.fragments.get_mut(&id).unwrap(),
            Label::Exit => &mut self.exit,
        }
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.get(self.cursor).instructions.push(instruction);
    }

    pub fn jump(&mut self, to: Label) {
        self.get(self.cursor).terminator = Some(Terminator::Jump(to));
        let cursor = self.cursor;
        self.get(to).predecessors.push(cursor);
    }

    pub fn branch(&mut self, cond: Var, to: Label, or: Label) {
        self.get(self.cursor).terminator = Some(Terminator::Branch(cond, to, or));
        let cursor = self.cursor;
        self.get(to).predecessors.push(cursor);
        self.get(or).predecessors.push(cursor);
    }

    pub fn ret(&mut self, var: Var) {
        self.get(self.cursor).terminator = Some(Terminator::Return(var));
    }

    pub fn phi(&mut self, label: Label, dst: Var, src: Vec<(Label, Var)>) {
        self.get(label).phis.push((dst, src));
    }

    pub fn seal(&mut self, label: Label) -> Result<(), Fault> {
        if self.sealed.contains(&label) {
            return Ok(());
        }
        if let Some(phis) = self.incompleted.remove(&label) {
            let preds = self.get(label).predecessors.clone();
            for (key, var) in phis {
                let mut operands = Vec::new();
                for pred in preds.clone() {
                    let v = self.lookup(pred, key)?;
                    operands.push((pred, v));
                }
                self.phi(label, var, operands);
            }
        }
        self.sealed.insert(label);
        Ok(())
    }

    pub fn define(&mut self, label: Label, id: Id, value: Var) {
        self.defs.entry(label).or_default().insert(id, value);
    }

    pub fn lookup(&mut self, label: Label, id: Id) -> Result<Var, Fault> {
        if let Some(var) = self.defs.get(&label).and_then(|x| x.get(&id)) {
            return Ok(*var);
        }

        let predecessors = self.get(label).predecessors.clone();
        let var = if !self.sealed.contains(&label) {
            let var = self.var();
            self.incompleted.entry(label).or_default().insert(id, var);
            var
        } else if predecessors.is_empty() {
            return Err(Fault::ValueNotDefined);
        } else if predecessors.len() == 1 {
            self.lookup(predecessors[0], id)?
        } else {
            let var = self.var();
            self.define(label, id, var);
            let mut operands = Vec::new();
            for pred in predecessors {
                let v = self.lookup(pred, id)?;
                operands.push((pred, v));
            }
            self.phi(label, var, operands);
            var
        };
        self.define(label, id, var);
        Ok(var)
    }
}

#[derive(Debug, Default)]
pub struct Fragment {
    pub phis: Vec<(Var, Vec<(Label, Var)>)>,
    pub predecessors: Vec<Label>,
    pub instructions: Vec<Instruction>,
    pub terminator: Option<Terminator>,
}

#[derive(Debug)]
pub enum Instruction {
    Field(Var, Var, usize),
    Func(Var, usize),
    Load(Var, Const),
    Binary(Var, Var, BinOp, Var),
    Unary(Var, UnaOp, Var),
    Call(Var, Var, Vec<Var>),
    List(Var, Vec<Var>),
    Tuple(Var, Vec<Var>),
    Index(Var, Var, Var),
    Method(Var, Var, usize, Vec<Var>),
    Group(Var, usize),
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

pub struct Dst(Option<Var>);

impl Dst {
    #[track_caller]
    pub fn var(&self) -> Result<Var, Fault> {
        self.0.ok_or(Fault::UnacceptableVoid(Location::caller()))
    }

    pub fn void() -> Self {
        Self(None)
    }
}

impl From<Var> for Dst {
    fn from(value: Var) -> Self {
        Self(Some(value))
    }
}
