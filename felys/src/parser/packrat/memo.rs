use crate::parser::registry::{Cache, Rule};
use std::collections::HashMap;

#[derive(Default)]
pub struct Memo {
    data: HashMap<(usize, bool, Rule), (usize, Cache)>,
}

impl Memo {
    pub fn get(&self, cur: usize, s: bool, rule: Rule) -> Option<(usize, Cache)> {
        self.data.get(&(cur, s, rule)).cloned()
    }

    pub fn insert(&mut self, cur: usize, s: bool, rule: Rule, end: usize, cache: Cache) {
        self.data.insert((cur, s, rule), (end, cache));
    }
}
