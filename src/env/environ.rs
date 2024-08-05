use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::Context;
use crate::env::Object;
use crate::error::RuntimeError;

pub struct Environ<'a> {
    pub builtin: &'a Scope,
    pub timer: &'a Receiver<bool>,
    pub called: (usize, usize),
    pub body: Vec<Scope>,
}


pub struct Scope {
    pub body: HashMap<String, Object>,
}


impl Environ<'_> {
    pub(super) fn get_object(&self, ident: &String) -> Result<&Object, RuntimeError> {
        for scope in self.body.iter().rev() {
            if let Some(object) = scope.body.get(ident) {
                return Ok(object);
            }
        }
        if let Some(object) = self.builtin.body.get(ident) {
            Ok(object)
        } else {
            Err(RuntimeError::ObjectDoesNotExist { s: ident.clone() })
        }
    }

    pub(super) fn call_object(
        &self, out: &mut Vec<String>,
        ident: &String, args: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let object = self.get_object(ident)?;
        if let Object::Function { args: names, body } = object {
            if names.len() != args.len() {
                return Err(RuntimeError::ArgsMappingFailed { s: ident.clone() });
            }
            let mut default = names.iter()
                .zip(args)
                .map(|(k, v)| (k.clone(), v))
                .collect::<HashMap<String, Object>>();
            default.insert(ident.clone(), object.clone());
            let mut env = self.sandbox(default)?;
            let result = if let Some(value) = body.run(&mut env, out)? {
                value
            } else {
                Object::None
            };
            Ok(result)
        } else if let Object::Rust(func) = object {
            let mut context = Context { args, out };
            Ok(func(&mut context).result?)
        } else {
            Err(RuntimeError::IdentNotCallable { s: ident.clone() })
        }
    }
}


impl Scope {
    pub fn new(body: HashMap<String, Object>) -> Self {
        Self { body }
    }

    pub(super) fn contains(&self, key: &String) -> bool {
        self.body.contains_key(key)
    }
}
