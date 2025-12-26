use crate::ast::{BinOp, UnaOp};
use crate::error::Fault;
use std::collections::HashMap;

pub struct Context {
    vars: usize,
    labels: usize,
    scopes: Vec<HashMap<usize, usize>>,
    pub loops: Vec<(Label, Label)>,
    pub writebacks: Vec<(Var, bool)>,
}

impl Context {
    pub fn new<'a>(args: impl Iterator<Item = &'a usize>) -> Self {
        let mut floor = HashMap::new();
        for (var, arg) in args.enumerate() {
            floor.insert(*arg, var);
        }
        Self {
            vars: floor.len(),
            labels: 1,
            scopes: vec![floor],
            loops: Vec::new(),
            writebacks: Vec::new(),
        }
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

    pub fn stack(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn unstack(&mut self) {
        self.scopes.pop();
    }

    pub fn add(&mut self, id: usize, var: Var) {
        self.scopes.last_mut().unwrap().insert(id, var);
    }

    pub fn get(&self, id: usize) -> Option<Var> {
        for scope in self.scopes.iter().rev() {
            if let Some(var) = scope.get(&id) {
                return Some(*var);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Function {
    pub segments: Vec<Segment>,
}

impl Function {
    pub fn new() -> Self {
        Self {
            segments: vec![Segment::new(0)],
        }
    }

    pub fn append(&mut self, label: usize) {
        self.segments.push(Segment::new(label));
    }

    pub fn field(&mut self, dst: Var, src: Var, offset: usize) {
        let ir = Instruction::Field(dst, src, offset);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }

    pub fn func(&mut self, dst: Var, id: usize) {
        let ir = Instruction::Func(dst, id);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }

    pub fn load(&mut self, dst: Var, id: usize) {
        let ir = Instruction::Load(dst, id);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }

    pub fn binary(&mut self, dst: Var, lhs: Var, op: BinOp, rhs: Var) {
        let ir = Instruction::Binary(dst, lhs, op, rhs);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }

    pub fn unary(&mut self, dst: Var, op: UnaOp, inner: Var) {
        let ir = Instruction::Unary(dst, op, inner);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }

    pub fn branch(&mut self, cond: Var, on: bool, label: Label) {
        let ir = Instruction::Branch(cond, on, label);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }

    pub fn jump(&mut self, label: Label) {
        let ir = Instruction::Jump(label);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }

    pub fn copy(&mut self, dst: Var, src: Var) {
        let ir = Instruction::Copy(dst, src);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }

    pub fn ret(&mut self, var: Option<Var>) {
        let ir = Instruction::Return(var);
        self.segments.last_mut().unwrap().instructions.push(ir);
    }
}

#[derive(Debug)]
pub struct Segment {
    label: Label,
    instructions: Vec<Instruction>,
}

impl Segment {
    pub fn new(label: usize) -> Self {
        Self {
            label,
            instructions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Field(Var, Var, usize),
    Func(Var, usize),
    Load(Var, usize),
    Binary(Var, Var, BinOp, Var),
    Unary(Var, UnaOp, Var),
    Copy(Var, Var),
    Branch(Var, bool, Label),
    Jump(Label),
    Return(Option<Var>),
}

pub struct Dst(Option<Var>);

impl Dst {
    pub fn var(&self) -> Result<Var, Fault> {
        self.0.ok_or(Fault::NoReturnValue)
    }

    pub fn none() -> Self {
        Self(None)
    }
}

impl From<Var> for Dst {
    fn from(value: Var) -> Self {
        Self(Some(value))
    }
}

pub type Var = usize;

pub type Label = usize;
