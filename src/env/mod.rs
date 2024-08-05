use std::collections::HashMap;

pub use crate::env::environ::*;
pub use crate::env::object::*;
use crate::error::RuntimeError;

mod object;
mod environ;


impl Environ<'_> {
    pub fn sandbox(&self, default: HashMap<String, Object>) -> Result<Self, RuntimeError> {
        if self.called.0 < self.called.1 {
            Ok(Self {
                builtin: self.builtin,
                timer: self.timer,
                body: vec![Scope::new(default)],
                called: (self.called.0+1, self.called.1),
            })
        } else {
            Err(RuntimeError::CallStackOverflow)
        }
    }

    pub fn store(&mut self, key: String, value: Object) {
        for scope in self.body.iter_mut() {
            if scope.contains(&key) {
                scope.body.insert(key, value);
                return;
            }
        }
        if let Some(scope) = self.body.last_mut() {
            scope.body.insert(key, value);
        }
    }

    pub fn eval(
        &self, out: &mut Vec<String>,
        ident: &String, args: Vec<Object>, callable: &bool,
    ) -> Result<Object, RuntimeError> {
        let result = if *callable {
            self.call_object(out, ident, args)?
        } else {
            self.get_object(ident)?.clone()
        };
        Ok(result)
    }

    pub fn shrink(&mut self) {
        self.body.pop();
    }

    pub fn expand(&mut self) {
        self.body.push(Scope::new(HashMap::new()))
    }
}