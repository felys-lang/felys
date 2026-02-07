use crate::utils::ast::Lit;
use crate::utils::function::{Const, Function, Instruction, Label, Phi, Terminator, Var};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Context {
    pub cursor: Label,
    pub consts: HashMap<Lit, Const>,
    ids: usize,
    f: Function,
    defs: HashMap<Label, HashMap<Id, Var>>,
    incompleted: HashMap<Label, HashMap<Id, Var>>,
    sealed: HashSet<Label>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Id {
    Interned(usize),
    Tmp(usize),
    Ret,
}

impl Context {
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

    pub fn push(&mut self, instruction: Instruction) {
        let fragment = self.f.get_mut(self.cursor).unwrap();
        if fragment.terminator.is_some() {
            return;
        }
        fragment.instructions.push(instruction);
    }

    fn dead(&self) -> bool {
        if self.cursor == Label::Entry {
            return false;
        }

        if self.sealed.contains(&self.cursor)
            && let Some(frag) = self.f.get(self.cursor)
            && frag.predecessors.is_empty()
        {
            true
        } else {
            false
        }
    }

    pub fn jump(&mut self, to: Label) -> bool {
        if self.dead() {
            return false;
        }
        let fragment = self.f.get_mut(self.cursor).unwrap();
        if fragment.terminator.is_some() {
            return false;
        }
        fragment.terminator = Some(Terminator::Jump(to));
        let cursor = self.cursor;
        self.f.get_mut(to).unwrap().predecessors.push(cursor);
        true
    }

    pub fn branch(&mut self, cond: Var, to: Label, or: Label) {
        if self.dead() {
            return;
        }
        let fragment = self.f.get_mut(self.cursor).unwrap();
        if fragment.terminator.is_some() {
            return;
        }
        fragment.terminator = Some(Terminator::Branch(cond, to, or));
        let cursor = self.cursor;
        self.f.get_mut(to).unwrap().predecessors.push(cursor);
        self.f.get_mut(or).unwrap().predecessors.push(cursor);
    }

    pub fn ret(&mut self, var: Var) {
        if self.dead() {
            return;
        }
        let fragment = self.f.get_mut(self.cursor).unwrap();
        if fragment.terminator.is_some() {
            return;
        }
        fragment.terminator = Some(Terminator::Return(var));
    }

    fn phi(&mut self, label: Label, var: Var, inputs: Vec<(Label, Var)>) {
        let phi = Phi { var, inputs };
        self.f.get_mut(label).unwrap().phis.push(phi);
    }

    pub fn seal(&mut self, label: Label) {
        if self.sealed.contains(&label) {
            return;
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
                let v = self.lookup(pred, id).unwrap();
                operands.push((pred, v));
            }
            self.phi(label, var, operands);
            var
        };
        self.define(label, id, var);
        Some(var)
    }
}

impl From<Context> for Function {
    fn from(value: Context) -> Self {
        value.f
    }
}
