use crate::rspegen::Intern;
use crate::runtime::context::value::Value;
use crate::runtime::shared::Signal;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

pub struct Backend<'a> {
    pub intern: &'a Intern,
    pub timer: &'a Receiver<bool>,
    pub depth: (u64, u64),
    pub data: Vec<HashMap<usize, Value>>,
}

impl Backend<'_> {
    pub fn sandbox(&self) -> Result<Self, Signal> {
        if self.depth.0 < self.depth.1 {
            Ok(Self {
                intern: self.intern,
                timer: self.timer,
                depth: (self.depth.0 + 1, self.depth.1),
                data: vec![HashMap::new()],
            })
        } else {
            Err(Signal::Error("stack overflow"))
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
        Err(Signal::Error("id does not exist"))
    }

    pub fn stack(&mut self) {
        self.data.push(HashMap::new())
    }

    pub fn unstack(&mut self) {
        self.data.pop();
    }
}
