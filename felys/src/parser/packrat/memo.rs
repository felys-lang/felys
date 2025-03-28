use crate::parser::registry::Cache;
use std::collections::HashMap;

#[derive(Default)]
pub struct Memo {
    data: HashMap<(usize, usize, bool), (usize, Cache)>,
}

impl Memo {
    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn get(&self, cur: usize, id: usize, s: bool) -> Option<(usize, Cache)> {
        self.data.get(&(cur, id, s)).cloned()
    }

    pub fn insert(&mut self, cur: usize, id: usize, s: bool, end: usize, cache: Cache) {
        self.data.insert((cur, id, s), (end, cache));
    }
}
