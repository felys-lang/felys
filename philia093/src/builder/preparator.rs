use crate::ast::{Alter, Assignment, Atom, Expect, Grammar, Hierarchy, Item, Lookahead, Rule, Tag};
use crate::builder::common::{Builder, Tags, Template};
use crate::philia093::Intern;
use std::collections::{HashMap, HashSet};

impl Tags {
    fn add(&mut self, tag: &Tag, name: usize) {
        match tag {
            Tag::Memo => self.memo.insert(name),
            Tag::Left => self.left.insert(name),
            Tag::Fast => self.fast.insert(name),
        };
    }
}

impl Builder {
    pub fn new(grammar: Grammar, intern: Intern) -> Self {
        let mut peg = HashMap::new();
        let mut rex = HashMap::new();
        let mut order = Vec::new();
        let mut keywords = Vec::new();
        let mut tags = Tags {
            memo: HashSet::new(),
            left: HashSet::new(),
            fast: HashSet::new(),
        };

        for callable in grammar.callables {
            match callable.hierarchy {
                Hierarchy::Peg(ty, rule) => {
                    keywords.append(&mut rule.keywords(&intern));
                    peg.insert(callable.name, (ty, rule));
                    order.push((callable.name, Template::Rule));
                }
                Hierarchy::Rex(regex) => {
                    rex.insert(callable.name, regex);
                    order.push((callable.name, Template::Lang));
                }
            };
            if let Some(decorator) = callable.deco {
                for tag in decorator.iter() {
                    tags.add(tag, callable.name);
                }
            }
        }

        let mut graph = HashMap::new();
        for (name, (_, rule)) in &peg {
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

        let mut languages = HashMap::new();
        for (name, regex) in &rex {
            if !languages.contains_key(name) {
                let language = regex.desugar(&rex, &mut languages, &intern);
                languages.insert(*name, language);
            }
        }

        Self {
            intern,
            tags,
            rules: peg,
            langs: languages,
            order,
            keywords,
            import: grammar.import,
        }
    }
}

impl Rule {
    fn left(&self) -> HashSet<usize> {
        let mut left = HashSet::new();
        for alter in self.0.iter() {
            left.extend(alter.left());
        }
        left
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        let mut keywords = Vec::new();
        for alter in self.0.iter() {
            keywords.extend(alter.keywords(intern));
        }
        keywords
    }
}

impl Alter {
    fn left(&self) -> HashSet<usize> {
        let mut left = HashSet::new();
        for assignment in self.assignments.iter() {
            left.extend(assignment.left());
            if assignment.truncated() {
                break;
            }
        }
        left
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        let mut keywords = Vec::new();
        for assignment in self.assignments.iter() {
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
            Assignment::Eof => HashSet::new(),
        }
    }

    fn truncated(&self) -> bool {
        match self {
            Assignment::Named(_, x) => x.truncated(),
            Assignment::Lookahead(_) => true,
            Assignment::Anonymous(x) => x.truncated(),
            Assignment::Clean => false,
            Assignment::Eof => false,
        }
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        match self {
            Assignment::Named(_, x) => x.keywords(intern),
            Assignment::Lookahead(x) => x.keywords(intern),
            Assignment::Anonymous(x) => x.keywords(intern),
            Assignment::Clean => Vec::new(),
            Assignment::Eof => Vec::new(),
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
            Item::Eager(x, _) => x.left(),
            Item::Repetition(x) => x.left(),
            Item::Optional(x) => x.left(),
            Item::Name(x) => x.left(),
        }
    }

    fn truncated(&self) -> bool {
        match self {
            Item::Eager(_, _) => true,
            Item::Repetition(_) => false,
            Item::Optional(_) => false,
            Item::Name(_) => true,
        }
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        match self {
            Item::Eager(x, _) => x.keywords(intern),
            Item::Repetition(x) => x.keywords(intern),
            Item::Optional(x) => x.keywords(intern),
            Item::Name(x) => x.keywords(intern),
        }
    }
}

impl Atom {
    fn left(&self) -> HashSet<usize> {
        match self {
            Atom::Name(name) => HashSet::from([*name]),
            Atom::Expect(_) => HashSet::new(),
            Atom::Nested(_) => HashSet::new(),
        }
    }

    fn keywords(&self, intern: &Intern) -> Vec<String> {
        match self {
            Atom::Name(_) => Vec::new(),
            Atom::Expect(expect) => expect.keywords(intern),
            Atom::Nested(rule) => rule.keywords(intern),
        }
    }
}

impl Expect {
    fn keywords(&self, intern: &Intern) -> Vec<String> {
        match self {
            Expect::Once(_) => Vec::new(),
            Expect::Keyword(x) => vec![x.squeeze(intern)],
        }
    }
}
