use std::collections::HashMap;
use std::rc::Rc;

pub struct Pool {
    pub(crate) body: HashMap<Rc<String>, usize>,
    pub(crate) fast: Vec<Rc<String>>,
}

impl Pool {
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