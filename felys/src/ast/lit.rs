#[derive(Clone, Debug)]
pub enum Lit {
    Int(usize),
    Float(usize),
    Bool(Bool),
    Str(Str),
}

#[derive(Clone, Debug)]
pub enum Bool {
    True,
    False,
}

pub type Str = Vec<Chunk>;

#[derive(Clone, Debug)]
pub enum Chunk {
    Slice(usize),
    Unicode(usize),
    Escape(usize),
}
