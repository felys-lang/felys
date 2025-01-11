use std::collections::HashMap;

pub struct Memo<R> {
    pub body: HashMap<(usize, bool, &'static str), (usize, R)>,
}

impl<R: Clone> Memo<R> {
    pub fn clear(&mut self) {
        self.body.clear()
    }

    pub fn get(&self, p: usize, m: bool, sig: &'static str) -> Option<(usize, R)> {
        self.body.get(&(p, m, sig)).cloned()
    }

    pub fn insert(&mut self, p: usize, m: bool, sig: &'static str, e: usize, res: R) {
        self.body.insert((p, m, sig), (e, res));
    }
}