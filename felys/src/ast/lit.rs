#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Lit {
    Int(usize),
    Float(usize),
    Bool(Bool),
    Str(Str),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Bool {
    True,
    False,
}

pub type Str = Vec<Chunk>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Chunk {
    Slice(usize),
    Unicode(usize),
    Escape(usize),
}
