use std::collections::HashMap;

pub struct Memo<R> {
    pub(crate) body: HashMap<(usize, bool, &'static str), (usize, R)>,
}

impl<R: Clone> Memo<R> {
    pub fn get(&self, p: usize, s: bool, t: &'static str) -> Option<(usize, R)> {
        self.body.get(&(p, s, t)).cloned()
    }

    pub fn insert(&mut self, p: usize, s: bool, t: &'static str, e: usize, res: R) {
        self.body.insert((p, s, t), (e, res));
    }
}