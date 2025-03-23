use std::collections::HashMap;
use std::rc::Rc;

pub struct Intern {
    data: HashMap<Rc<String>, usize>,
    fast: Vec<Rc<String>>,
}

impl Intern {
    pub fn id(&mut self, s: String) -> usize {
        if let Some(&id) = self.data.get(&s) {
            id
        } else {
            let key = Rc::new(s);
            let id = self.fast.len();
            self.fast.push(key.clone());
            self.data.insert(key, id);
            id
        }
    }

    pub fn get(&self, id: usize) -> Option<String> {
        let string = self.fast.get(id)?;
        Some((**string).clone())
    }
}
