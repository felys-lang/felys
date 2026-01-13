use crate::ast::{BinOp, Lit, UnaOp};
use crate::cyrene::fault::Fault;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::ops::Range;
use std::rc::Rc;

pub struct Context {
    pub cursor: Label,
    pub cache: HashMap<Lit, Const>,
    f: Function,
    ids: usize,
    defs: HashMap<Label, HashMap<Id, Var>>,
    incompleted: HashMap<Label, HashMap<Id, Var>>,
    sealed: HashSet<Label>,
}

#[derive(Debug)]
pub struct Function {
    pub args: Range<usize>,
    pub vars: usize,
    pub labels: usize,
    pub entry: Fragment,
    pub fragments: HashMap<usize, Fragment>,
    pub exit: Fragment,
}

impl Function {
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

    pub fn add(&mut self, label: Label) -> &mut Fragment {
        let Label::Id(id) = label else { panic!() };
        self.fragments.entry(id).or_default()
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
    pub fn new(args: usize) -> Self {
        Self {
            cursor: Default::default(),
            cache: Default::default(),
            f: Function {
                args: 0..args,
                vars: 0,
                labels: 0,
                entry: Default::default(),
                fragments: Default::default(),
                exit: Default::default(),
            },
            ids: 0,
            defs: Default::default(),
            incompleted: Default::default(),
            sealed: Default::default(),
        }
    }

    pub fn export(self) -> Function {
        self.f
    }

    pub fn id(&mut self) -> Id {
        let id = self.ids;
        self.ids += 1;
        Id::Tmp(id)
    }

    pub fn var(&mut self) -> Var {
        self.f.var()
    }

    pub fn label(&mut self) -> Label {
        self.f.label()
    }

    pub fn add(&mut self, label: Label) {
        self.f.add(label);
    }

    pub fn unreachable(&mut self) -> Result<Option<Var>, Fault> {
        let dead = self.f.label();
        self.add(dead);
        self.cursor = dead;
        Ok(None)
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.f
            .modify(self.cursor)
            .unwrap()
            .instructions
            .push(instruction);
    }

    pub fn jump(&mut self, to: Label) {
        self.f.modify(self.cursor).unwrap().terminator = Some(Terminator::Jump(to));
        let cursor = self.cursor;
        self.f.modify(to).unwrap().predecessors.push(cursor);
    }

    pub fn branch(&mut self, cond: Var, to: Label, or: Label) {
        self.f.modify(self.cursor).unwrap().terminator = Some(Terminator::Branch(cond, to, or));
        let cursor = self.cursor;
        self.f.modify(to).unwrap().predecessors.push(cursor);
        self.f.modify(or).unwrap().predecessors.push(cursor);
    }

    pub fn ret(&mut self, var: Var) {
        self.f.modify(self.cursor).unwrap().terminator = Some(Terminator::Return(var));
    }

    pub fn phi(&mut self, label: Label, dst: Var, src: Vec<(Label, Var)>) {
        self.f.modify(label).unwrap().phis.push((dst, src));
    }

    pub fn seal(&mut self, label: Label) -> Result<(), Fault> {
        if self.sealed.contains(&label) {
            return Ok(());
        }
        if let Some(phis) = self.incompleted.remove(&label) {
            let preds = self.f.get(label).unwrap().predecessors.clone();
            for (key, var) in phis {
                let mut operands = Vec::new();
                for pred in preds.clone() {
                    let v = self.lookup(pred, key).unwrap();
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

    pub fn lookup(&mut self, label: Label, id: Id) -> Option<Var> {
        if let Some(var) = self.defs.get(&label).and_then(|x| x.get(&id)) {
            return Some(*var);
        }

        let predecessors = self.f.get(label).unwrap().predecessors.clone();
        let var = if !self.sealed.contains(&label) {
            let var = self.f.var();
            self.incompleted.entry(label).or_default().insert(id, var);
            var
        } else if predecessors.is_empty() {
            return None;
        } else if predecessors.len() == 1 {
            self.lookup(predecessors[0], id)?
        } else {
            let var = self.f.var();
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
        Some(var)
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
    Unpack(Var, Var, usize),
    Group(Var, usize),
    Function(Var, usize),
    Load(Var, Const),
    Binary(Var, Var, BinOp, Var),
    Unary(Var, UnaOp, Var),
    Call(Var, Var, Vec<Var>),
    List(Var, Vec<Var>),
    Tuple(Var, Vec<Var>),
    Index(Var, Var, Var),
    Method(Var, Var, usize, Vec<Var>),
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
