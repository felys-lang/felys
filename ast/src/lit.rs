use crate::Symbol;

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

pub enum Int {
    Base16(Symbol),
    Base10(Symbol),
    Base8(Symbol),
    Base2(Symbol),
}

pub type Float = Symbol;

pub enum Bool {
    True,
    False,
}

pub type Str = Symbol;