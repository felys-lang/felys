use crate::utils::BufVec;
use std::rc::Rc;

pub struct Grammar {
    pub import: Option<usize>,
    pub callables: Vec<Callable>,
}

pub enum Callable {
    Rule(Option<BufVec<Tag, 1>>, Prefix, usize, Option<usize>, Rule),
    Regex(Option<BufVec<Tag, 1>>, usize, Regex),
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
pub enum Tag {
    Memo,
    Left,
    Whitespace,
}

pub struct Alter {
    pub assignments: BufVec<Assignment, 1>,
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
    Eager(Atom, Option<Message>),
    Optional(Atom),
    Repetition(Atom),
    Name(Atom),
}

pub enum Message {
    Parentheses(Vec<Message>),
    Segment(usize),
}

pub enum Atom {
    Name(usize),
    Expect(Expect),
    Nested(Rule),
}

pub enum Expect {
    Once(BufVec<usize, 1>),
    Keyword(BufVec<usize, 1>),
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
    Exclude(BufVec<(usize, usize), 1>),
    Include(BufVec<(usize, usize), 1>),
    Literal(BufVec<usize, 1>),
    Name(usize),
}
