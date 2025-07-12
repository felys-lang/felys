use std::rc::Rc;

pub struct Grammar {
    pub import: Option<usize>,
    pub callables: Vec<Callable>,
}

pub enum Callable {
    Rule(Option<Decorator>, Prefix, usize, usize, Rule),
    Regex(Option<Decorator>, usize, Regex),
    Shared(Decorator, Vec<(usize, Regex)>),
}

pub enum Prefix {
    Peg,
    Lex,
}

pub struct Rule {
    pub first: Alter,
    pub more: Vec<Alter>,
}

#[derive(Debug)]
pub struct Decorator {
    pub first: Tag,
    pub more: Vec<Tag>,
}

#[derive(Debug)]
pub enum Tag {
    Memo,
    Left,
    Intern,
    Whitespace,
}

pub struct Alter {
    pub assignments: Vec<Assignment>,
    pub action: Option<usize>,
}

pub enum Assignment {
    Named(usize, Item),
    Lookahead(Lookahead),
    Anonymous(Item),
    Clean,
}

pub enum Lookahead {
    Positive(Atom),
    Negative(Atom),
}

pub enum Item {
    Optional(Atom),
    ZeroOrMore(Atom),
    OnceOrMore(bool, Atom),
    Name(bool, Atom),
}

pub enum Atom {
    Name(usize),
    Expect(Expect),
    Nested(Rule),
}

pub enum Expect {
    Once(usize),
    Keyword(usize),
}

#[derive(Clone)]
pub enum Regex {
    Union(Rc<Regex>, Rc<Regex>),
    Concat(Rc<Regex>, Rc<Regex>),
    ZeroOrMore(Rc<Regex>),
    OnceOrMore(Rc<Regex>),
    Primary(Primary),
}

#[derive(Clone)]
pub enum Primary {
    Parentheses(Rc<Regex>),
    Exclude(Vec<(usize, usize)>),
    Include(Vec<(usize, usize)>),
    Literal(Vec<usize>),
    Name(usize),
}
