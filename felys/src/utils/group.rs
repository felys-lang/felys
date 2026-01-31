use crate::utils::bytecode::{Id, Index};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Group {
    pub indices: HashMap<Id, Index>,
    pub methods: HashMap<Id, Index>,
}

impl Group {
    pub fn new(fields: Vec<usize>) -> Self {
        let mut indices = HashMap::new();
        for (i, field) in fields.iter().enumerate() {
            indices.insert(Id::try_from(*field).unwrap(), Index::try_from(i).unwrap());
        }
        Self {
            indices,
            methods: HashMap::new(),
        }
    }

    pub fn attach(&mut self, id: usize, index: usize) {
        self.methods
            .insert(Id::try_from(id).unwrap(), Index::try_from(index).unwrap());
    }
}
