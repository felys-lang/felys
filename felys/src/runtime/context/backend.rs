use crate::ast::Ident;
use crate::nn::optim::Optimizer;
use crate::parser::Intern;
use crate::runtime::context::value::Value;
use crate::runtime::shared::Signal;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Global<'a> {
    pub optim: &'a mut Optimizer,
    pub stdout: &'a mut Vec<String>,
    pub constants: &'a HashMap<usize, Value>,
    pub intern: &'a Intern,
}

pub struct Frame {
    pub depth: (usize, usize),
    pub data: Vec<HashMap<usize, Value>>,
}

impl Frame {
    pub fn sandbox(&self) -> Result<Self, Signal> {
        if self.depth.0 < self.depth.1 {
            Ok(Self {
                depth: (self.depth.0 + 1, self.depth.1),
                data: vec![HashMap::new()],
            })
        } else {
            Err(Signal::Error("stack overflow".to_string()))
        }
    }

    pub fn put(&mut self, key: usize, value: Value) {
        for floor in self.data.iter_mut() {
            if let Entry::Occupied(mut e) = floor.entry(key) {
                e.insert(value);
                return;
            }
        }
        if let Some(scope) = self.data.last_mut() {
            scope.insert(key, value);
        }
    }

    pub fn get(&self, key: usize) -> Result<Value, Signal> {
        for floor in self.data.iter().rev() {
            if let Some(value) = floor.get(&key) {
                return Ok(value.clone());
            }
        }
        Err(Signal::Error("id does not exist".to_string()))
    }

    pub fn stack(&mut self, default: Vec<(Ident, Value)>) {
        let mut data = HashMap::new();
        for (k, v) in default {
            data.insert(k, v);
        }
        self.data.push(data)
    }

    pub fn unstack(&mut self) {
        self.data.pop();
    }
}
