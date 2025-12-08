use crate::ast::{Primary, Regex};
use crate::builder::common::s2c;
use crate::builder::dfa::common::{Language, Terminal};
use crate::philia093::Intern;
use std::collections::HashMap;

impl Regex {
    pub fn desugar(
        &self,
        regexes: &HashMap<usize, Regex>,
        languages: &mut HashMap<usize, Language>,
        intern: &Intern,
    ) -> Language {
        match self {
            Regex::Union(c1, c2) => Language::Union(
                c1.desugar(regexes, languages, intern).into(),
                c2.desugar(regexes, languages, intern).into(),
            ),
            Regex::Concat(c1, c2) => Language::Concat(
                c1.desugar(regexes, languages, intern).into(),
                c2.desugar(regexes, languages, intern).into(),
            ),
            Regex::ZeroOrMore(c) => Language::Kleene(c.desugar(regexes, languages, intern).into()),
            Regex::OnceOrMore(c) => {
                let c = c.desugar(regexes, languages, intern);
                Language::Concat(c.clone().into(), Language::Kleene(c.into()).into())
            }
            Regex::Primary(c) => c.desugar(regexes, languages, intern),
        }
    }
}

impl Primary {
    pub fn desugar(
        &self,
        regexes: &HashMap<usize, Regex>,
        languages: &mut HashMap<usize, Language>,
        intern: &Intern,
    ) -> Language {
        match self {
            Primary::Parentheses(c) => c.desugar(regexes, languages, intern),
            Primary::Exclude(set) => {
                let mut set = set
                    .iter()
                    .map(|(s, e)| {
                        let s = s2c(intern.get(s).unwrap()) as usize;
                        let e = s2c(intern.get(e).unwrap()) as usize;
                        if s < e { (s, e) } else { (e, s) }
                    })
                    .collect::<Vec<_>>();
                set.sort_by_key(|&(s, _)| s);
                let mut merged = Vec::new();
                for (start, end) in set {
                    let Some((_, last)) = merged.last_mut() else {
                        merged.push((start, end));
                        continue;
                    };
                    if start <= last.saturating_add(1) {
                        *last = end;
                    } else {
                        merged.push((start, end));
                    }
                }
                let mut include = Vec::new();
                let mut current = usize::MIN;
                for (start, end) in merged {
                    if current < start {
                        include.push((current, start.saturating_sub(1)));
                    }
                    current = end.saturating_add(1);
                }
                if current < usize::MAX {
                    include.push((current, usize::MAX));
                }
                Language::Terminal(Terminal::Set(include), 0)
            }
            Primary::Include(set) => {
                let set = set
                    .iter()
                    .map(|(s, e)| {
                        let s = s2c(intern.get(s).unwrap()) as usize;
                        let e = s2c(intern.get(e).unwrap()) as usize;
                        if s < e { (s, e) } else { (e, s) }
                    })
                    .collect();
                Language::Terminal(Terminal::Set(set), 0)
            }
            Primary::Literal(literal) => {
                let mut chars = literal.iter().map(|x| s2c(intern.get(x).unwrap()));
                let first = chars.next().unwrap() as usize;
                let mut node = Language::Terminal(Terminal::Set(vec![(first, first)]), 0);
                for c in chars {
                    node = Language::Concat(
                        node.into(),
                        Language::Terminal(Terminal::Set(vec![(c as usize, c as usize)]), 0).into(),
                    );
                }
                node
            }
            Primary::Name(name) => {
                if let Some(language) = languages.get(name) {
                    Language::Nested(language.clone().into())
                } else {
                    let regex = &regexes.get(name).unwrap();
                    let language = regex.desugar(regexes, languages, intern);
                    languages.insert(*name, language.clone());
                    Language::Nested(language.into())
                }
            }
        }
    }
}
