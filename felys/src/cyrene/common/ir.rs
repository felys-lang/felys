use crate::ast::{BinOp, UnaOp};
use crate::error::Fault;
use std::collections::HashMap;

pub struct Context {
    vars: usize,
    labels: usize,
    pub loops: Vec<(Label, Label)>,
    pub writebacks: Vec<(Var, bool)>,
}

impl Context {
    pub fn new(args: HashMap<usize, usize>) -> Self {
        Self {
            vars: args.len(),
            labels: 1,
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
}

pub struct Scope {
    data: Vec<HashMap<usize, usize>>,
}

impl Scope {}

#[derive(Debug)]
pub struct Function {
    segments: Vec<Segment>,
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

    pub fn load(&mut self, dst: Var, label: usize) {
        let ir = Instruction::Load(dst, label);
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
