use std::collections::{HashMap, HashSet};
use std::rc::Rc;

pub struct Pool {
    pub body: HashMap<Rc<String>, usize>,
    pub fast: Vec<Rc<String>>,
    pub keywords: HashSet<&'static str>,
}

impl Pool {
    pub fn keyword(&self, ident: &str) -> bool {
        self.keywords.contains(ident)
    }

    pub fn id(&mut self, s: String) -> usize {
        if let Some(&id) = self.body.get(&s) {
            id
        } else {
            let key = Rc::new(s);
            let id = self.fast.len();
            self.fast.push(key.clone());
            self.body.insert(key, id);
            id
        }
    }

    pub fn get(&self, id: usize) -> Option<String> {
        let string = self.fast.get(id)?;
        Some((**string).clone())
    }
}