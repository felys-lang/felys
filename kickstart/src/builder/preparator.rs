use crate::ast::{Alter, Assignment, Atom, Callable, Grammar, Item, Lookahead, Rule, Tag};
use crate::builder::common::{s2c, Builder, Tags};
use crate::parser::Intern;
use std::collections::{HashMap, HashSet};

impl Tags {
    fn add(&mut self, tag: &Tag, name: usize) {
        match tag {
            Tag::Memo => self.memo.insert(name),
            Tag::Left => self.left.insert(name),
            Tag::Intern => self.intern.insert(name),
            Tag::Token => self.token.insert(name),
            Tag::Whitespace => self.ws.insert(name),
        };
    }
}

impl Builder {
    pub fn new(grammar: Grammar, intern: Intern) -> Self {
        let mut rules = HashMap::new();
        let mut languages = HashMap::new();
        let mut regexes = HashMap::new();
        let mut keywords = Vec::new();
        let mut sequence = Vec::new();
        let mut tags = Tags {
            memo: HashSet::new(),
            left: HashSet::new(),
            token: HashSet::new(),
            intern: HashSet::new(),
            ws: HashSet::new(),
        };

        for callable in grammar.callables {
            let (name, deco) = match callable {
                Callable::Rule(deco, prefix, name, ty, rule) => {
                    keywords.append(&mut rule.keywords(&intern));
                    rules.insert(name, (prefix, ty, rule));
                    (name, deco)
                }
                Callable::Regex(deco, name, regex) => {
                    regexes.insert(name, regex);
                    (name, deco)
                }
                Callable::Shared(deco, shared) => {
                    for (name, regex) in shared {
                        sequence.push(name);
                        regexes.insert(name, regex);
                        tags.add(&deco.first, name);
                        for tag in &deco.more {
                            tags.add(tag, name);
                        }
                    }
                    continue;
                }
            };
            sequence.push(name);
            let Some(decorator) = deco else {
                continue;
            };
            tags.add(&decorator.first, name);
            for tag in &decorator.more {
                tags.add(tag, name);
            }
        }

        let mut graph = HashMap::new();
        for (name, (_, _, rule)) in &rules {
            graph.insert(*name, rule.left());
        }

        for (name, edges) in &graph {
            let mut todo = Vec::new();
            let mut visited = HashSet::new();
            for edge in edges {
                todo.push(edge)
            }
            while let Some(test) = todo.pop() {
                if visited.contains(test) {
                    continue;
                }
                if test == name {
                    tags.left.insert(*test);
                    break;
                }
                visited.insert(test);
                let Some(edges) = graph.get(test) else {
                    continue;
                };
                for edge in edges {
                    todo.push(edge)
                }
            }
        }

        for (name, regex) in &regexes {
            if !languages.contains_key(name) {
                let language = regex.desugar(&regexes, &mut languages, &intern);
                languages.insert(*name, language);
            }
        }

        Self {
            intern,
            tags,
            rules,
            languages,
            sequence,
            keywords,
            import: grammar.import,
        }
    }
}

impl Rule {
    fn left(&self) -> HashSet<usize> {
        let mut left = self.first.left();
        for alter in &self.more {
            left.extend(alter.left());
        }
        left
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        let mut keywords = self.first.keywords(intern);
        for alter in &self.more {
            keywords.extend(alter.keywords(intern));
        }
        keywords
    }
}

impl Alter {
    fn left(&self) -> HashSet<usize> {
        let mut left = HashSet::new();
        for assignment in &self.assignments {
            left.extend(assignment.left());
            if assignment.truncated() {
                break;
            }
        }
        left
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        let mut keywords = Vec::new();
        for assignment in &self.assignments {
            keywords.extend(assignment.keywords(intern));
        }
        keywords
    }
}

impl Assignment {
    fn left(&self) -> HashSet<usize> {
        match self {
            Assignment::Named(_, x) => x.left(),
            Assignment::Lookahead(x) => x.left(),
            Assignment::Anonymous(x) => x.left(),
            Assignment::Clean => HashSet::new(),
        }
    }

    fn truncated(&self) -> bool {
        match self {
            Assignment::Named(_, x) => x.truncated(),
            Assignment::Lookahead(x) => x.truncated(),
            Assignment::Anonymous(x) => x.truncated(),
            Assignment::Clean => false,
        }
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        match self {
            Assignment::Named(_, x) => x.keywords(intern),
            Assignment::Lookahead(x) => x.keywords(intern),
            Assignment::Anonymous(x) => x.keywords(intern),
            Assignment::Clean => Vec::new(),
        }
    }
}

impl Lookahead {
    fn left(&self) -> HashSet<usize> {
        match self {
            Lookahead::Positive(x) => x.left(),
            Lookahead::Negative(x) => x.left(),
        }
    }

    fn truncated(&self) -> bool {
        match self {
            Lookahead::Positive(_) => true,
            Lookahead::Negative(_) => true,
        }
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        match self {
            Lookahead::Positive(x) => x.keywords(intern),
            Lookahead::Negative(x) => x.keywords(intern),
        }
    }
}

impl Item {
    fn left(&self) -> HashSet<usize> {
        match self {
            Item::OnceOrMore(_, x) => x.left(),
            Item::ZeroOrMore(x) => x.left(),
            Item::Optional(x) => x.left(),
            Item::Name(_, x) => x.left(),
        }
    }

    fn truncated(&self) -> bool {
        match self {
            Item::OnceOrMore(_, _) => true,
            Item::ZeroOrMore(_) => false,
            Item::Optional(_) => false,
            Item::Name(_, _) => true,
        }
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        match self {
            Item::OnceOrMore(_, x) => x.keywords(intern),
            Item::ZeroOrMore(x) => x.keywords(intern),
            Item::Optional(x) => x.keywords(intern),
            Item::Name(_, x) => x.keywords(intern),
        }
    }
}

impl Atom {
    fn left(&self) -> HashSet<usize> {
        match self {
            Atom::Name(name) => HashSet::from([*name]),
            Atom::String(_) => HashSet::new(),
            Atom::Nested(_, _) => HashSet::new(),
        }
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        match self {
            Atom::Name(_) => Vec::new(),
            Atom::String(keyword) => {
                let string = keyword
                    .iter()
                    .map(|x| s2c(intern.get(x).unwrap()))
                    .collect();
                vec![string]
            }
            Atom::Nested(_, rule) => rule.keywords(intern),
        }
    }
}
