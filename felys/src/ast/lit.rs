#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Lit {
    Int(usize),
    Float(usize, usize),
    Bool(Bool),
    Str(Vec<Chunk>),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Bool {
    True,
    False,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Chunk {
    Slice(usize),
    Unicode(usize),
    Escape(usize),
}
