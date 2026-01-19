use std::collections::HashMap;

#[derive(Debug)]
pub struct Group {
    pub fields: Box<[usize]>,
    pub indices: HashMap<usize, usize>,
    pub methods: HashMap<usize, usize>,
}

impl Group {
    pub fn new(fields: Vec<usize>) -> Self {
        let mut indices = HashMap::new();
        for (i, field) in fields.iter().enumerate() {
            indices.insert(*field, i);
        }
        Self {
            fields: fields.into_boxed_slice(),
            indices,
            methods: HashMap::new(),
        }
    }
}
