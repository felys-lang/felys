mod object;
mod environ;


use std::collections::HashMap;
pub use crate::env::object::*;
pub use crate::env::environ::*;
use crate::error::RuntimeError;


impl Environ<'_> {
    pub fn sandbox(&self, default: HashMap<String, Object>) -> Self {
        Self {
            builtin: self.builtin, 
            timer: self.timer,
            body: vec![Scope::new(default)]
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
        ident: &String, args: Vec<Object>, callable: &bool
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