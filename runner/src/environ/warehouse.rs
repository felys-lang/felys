use crate::environ::Value;
use crate::execute::Signal;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Warehouse {
    pub floors: Vec<HashMap<usize, Value>>,
}

impl Warehouse {
    pub fn put(&mut self, key: usize, value: Value) {
        for floor in self.floors.iter_mut() {
            if let Entry::Occupied(mut e) = floor.entry(key) {
                e.insert(value);
                return;
            }
        }
        if let Some(scope) = self.floors.last_mut() {
            scope.insert(key, value);
        }
    }

    pub fn get(&self, key: usize) -> Result<Value, Signal> {
        for floor in self.floors.iter().rev() {
            if let Some(value) = floor.get(&key) {
                return Ok(value.clone());
            }
        }
        Err(Signal::Error("id does not exist"))
    }

    pub fn stack(&mut self) {
        self.floors.push(HashMap::new())
    }

    pub fn unstack(&mut self) {
        self.floors.pop();
    }
}