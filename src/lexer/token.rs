#[derive(PartialEq, Debug, Clone)]
pub enum KeywordType {
    While,
    If,
    Elif,
    Else,
    Return,
}


#[derive(PartialEq, Debug, Clone)]
pub enum ValueType {
    Boolean,
    String,
    Number,
    None,
}


#[derive(PartialEq, Debug, Clone)]
pub enum BinoptrType {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    And,
    Xor,
    Or,
}


#[derive(PartialEq, Debug, Clone)]
pub enum AssignType {
    Ade,
    Sue,
    Mue,
    Die,
    Moe,
    Asn,
}


#[derive(PartialEq, Debug, Clone)]
pub enum UnaoptrType {
    Not,
    Pos,
    Neg,
}


#[derive(PartialEq, Debug, Clone)]
pub enum SymbolType {
    LBrace,
    RBrace,
    LParen,
    RParen,
    Semicol,
    Comma,
    Pipe,
}


#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Val(ValueType),
    Key(KeywordType),
    Bin(BinoptrType),
    Una(UnaoptrType),
    Sym(SymbolType),
    Asn(AssignType),
    Identifier,
}


pub struct Token {
    pub kind: TokenType,
    pub value: String,
}


impl Token {
    pub fn new(t: TokenType, v: String) -> Self {
        Self { kind: t, value: v }
    }
}


impl From<KeywordType> for TokenType {
    fn from(value: KeywordType) -> Self {
        TokenType::Key(value)
    }
}

impl From<ValueType> for TokenType {
    fn from(value: ValueType) -> Self {
        TokenType::Val(value)
    }
}

impl From<BinoptrType> for TokenType {
    fn from(value: BinoptrType) -> Self {
        TokenType::Bin(value)
    }
}

impl From<UnaoptrType> for TokenType {
    fn from(value: UnaoptrType) -> Self {
        TokenType::Una(value)
    }
}

impl From<SymbolType> for TokenType {
    fn from(value: SymbolType) -> Self {
        TokenType::Sym(value)
    }
}

impl From<AssignType> for TokenType {
    fn from(value: AssignType) -> Self {
        TokenType::Asn(value)
    }
}
