use crate::parser::registry::Cache;
use std::collections::HashMap;

#[derive(Default)]
pub struct Memo {
    data: HashMap<(usize, bool, &'static str), (usize, Cache)>,
}

impl Memo {
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    pub fn get(&self, cur: usize, s: bool, sig: &'static str) -> Option<(usize, Cache)> {
        self.data.get(&(cur, s, sig)).cloned()
    }

    pub fn insert(&mut self, cur: usize, s: bool, sig: &'static str, end: usize, cache: Cache) {
        self.data.insert((cur, s, sig), (end, cache));
    }
}
