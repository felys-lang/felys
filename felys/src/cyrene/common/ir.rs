use crate::ast::{BinOp, Lit, UnaOp};
use crate::error::Fault;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug)]
pub struct Function {
    pub fragments: HashMap<Label, Fragment>,
}

pub struct Context {
    ids: usize,
    vars: usize,
    labels: usize,
    pub cursor: Label,
    pub cache: HashMap<Lit, Const>,
    pub fragments: HashMap<Label, Fragment>,
    pub defs: HashMap<Label, HashMap<Id, Var>>,
    pub incompleted: HashMap<Label, HashMap<Id, Var>>,
    pub sealed: HashSet<Label>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Id {
    Interned(usize),
    Tmp(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Const {
    Int(usize),
    Float(usize, usize),
    Bool(bool),
    Str(Rc<str>),
}

impl Context {
    pub fn new<'a>(args: impl Iterator<Item = &'a usize>) -> Self {
        let mut ctx = Self {
            ids: 0,
            cursor: 0,
            vars: 0,
            labels: 0,
            cache: HashMap::new(),
            fragments: HashMap::new(),
            defs: HashMap::new(),
            incompleted: HashMap::new(),
            sealed: HashSet::new(),
        };
        let entry = ctx.label();
        ctx.add(entry);
        ctx.seal(entry).unwrap();
        for (var, id) in args.enumerate() {
            ctx.define(ctx.cursor, Id::Interned(*id), var);
        }
        ctx
    }

    pub fn export(self) -> Function {
        Function {
            fragments: self.fragments,
        }
    }

    pub fn id(&mut self) -> Id {
        let id = Id::Interned(self.ids);
        self.ids += 1;
        id
    }

    pub fn var(&mut self) -> Var {
        let var = self.vars;
        self.vars += 1;
        var
    }

    pub fn label(&mut self) -> Var {
        let label = self.labels;
        self.labels += 1;
        label
    }

    pub fn unreachable(&mut self) -> Result<Dst, Fault> {
        let dead = self.label();
        self.add(dead);
        self.cursor = dead;
        Ok(Dst::void())
    }

    pub fn add(&mut self, label: Label) {
        self.fragments.insert(label, Fragment::default());
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.fragments
            .get_mut(&self.cursor)
            .unwrap()
            .instructions
            .push(instruction);
    }

    pub fn jump(&mut self, to: Label) {
        self.push(Instruction::Jump(to));
        self.fragments
            .get_mut(&to)
            .unwrap()
            .predecessors
            .push(self.cursor);
    }

    pub fn branch(&mut self, cond: Var, to: Label, or: Label) {
        self.push(Instruction::Branch(cond, to, or));
        self.fragments
            .get_mut(&to)
            .unwrap()
            .predecessors
            .push(self.cursor);
        self.fragments
            .get_mut(&or)
            .unwrap()
            .predecessors
            .push(self.cursor);
    }

    pub fn phi(&mut self, label: Label, dst: Var, src: Vec<(Label, Var)>) {
        self.fragments
            .get_mut(&label)
            .unwrap()
            .phis
            .push((dst, src));
    }

    pub fn seal(&mut self, label: Label) -> Result<(), Fault> {
        if self.sealed.contains(&label) {
            return Ok(());
        }
        if let Some(phis) = self.incompleted.remove(&label) {
            let preds = self.fragments.get(&label).unwrap().predecessors.clone();
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

        let predecessors = self.fragments.get(&label).unwrap().predecessors.clone();
        let var = if !self.sealed.contains(&label) {
            let var = self.var();
            self.incompleted.entry(label).or_default().insert(id, var);
            var
        } else if predecessors.is_empty() {
            return Err(Fault::NoReturnValue);
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
}

#[derive(Debug)]
pub enum Instruction {
    Field(Var, Var, usize),
    Func(Var, usize),
    Load(Var, Const),
    Binary(Var, Var, BinOp, Var),
    Unary(Var, UnaOp, Var),
    Copy(Var, Var),
    Branch(Var, Label, Label),
    Jump(Label),
    Return(Var),
    Buffer,
    Push(Var),
    Call(Var, Var),
    List(Var),
    Tuple(Var),
    Index(Var, Var, Var),
    Method(Var, Var, usize),
    Group(Var, usize),
}

pub type Var = usize;

pub type Label = usize;

pub struct Dst(Option<Var>);

impl Dst {
    pub fn var(&self) -> Result<Var, Fault> {
        self.0.ok_or(Fault::NoReturnValue)
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
