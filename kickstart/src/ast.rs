use crate::utils::BufVec;
use std::rc::Rc;

pub struct Grammar {
    pub import: Option<Action>,
    pub callables: Vec<Callable>,
}

pub struct Action(pub Vec<Nested>);

pub enum Callable {
    Peg(Option<BufVec<Tag, 1>>, usize, Option<Action>, Rule),
    Lex(Option<BufVec<Tag, 1>>, usize, Option<Action>, Rule),
    Rex(Option<BufVec<Tag, 1>>, usize, Regex),
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
    pub action: Option<Action>,
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

pub struct Message(pub Vec<Nested>);

pub enum Nested {
    Inner(Vec<Nested>),
    Segment(usize),
}

pub enum Atom {
    Name(usize),
    External(usize),
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
