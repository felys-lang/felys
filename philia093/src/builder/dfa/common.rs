#[derive(Clone)]
pub enum Language {
    Union(Box<Language>, Box<Language>),
    Concat(Box<Language>, Box<Language>),
    Kleene(Box<Language>),
    Nested(Box<Language>),
    Terminal(Terminal, usize),
}

#[derive(Clone)]
pub enum Terminal {
    Set(Vec<(usize, usize)>),
    Pound,
}

pub struct Automaton {
    pub transition: Vec<(usize, (usize, usize), usize)>,
    pub acceptance: Vec<bool>,
}
