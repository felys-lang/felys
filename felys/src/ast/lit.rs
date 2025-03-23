use crate::ast::utils::Id;

#[derive(Clone, Debug)]
pub enum Lit {
    /// integer: `0xf`, `0o77`, `15`, `0b1111`
    Int(Int),
    /// decimal: `11.11`
    Float(Float),
    /// boolean: `true`, `false`
    Bool(Bool),
    /// string: `"elysia"`, `f"{1+1} = 2"`, `r"\t\r\n"`
    Str(Str),
}

#[derive(Clone, Debug)]
pub enum Int {
    Base16(Id),
    Base10(Id),
    Base8(Id),
    Base2(Id),
}

pub type Float = Id;

#[derive(Clone, Debug)]
pub enum Bool {
    True,
    False,
}

pub type Str = Id;
