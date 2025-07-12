use crate::ast::{Prefix, Rule};
use crate::builder::dfa::common::Language;
use crate::parser::Intern;
use proc_macro2::TokenStream;
use std::collections::{HashMap, HashSet};

pub struct Builder {
    pub intern: Intern,
    pub tags: Tags,
    pub rules: HashMap<usize, (Prefix, usize, Rule)>,
    pub languages: HashMap<usize, Language>,
    pub sequence: Vec<usize>,
    pub keywords: Vec<String>,
    pub import: Option<usize>,
}

pub struct Tags {
    pub memo: HashSet<usize>,
    pub left: HashSet<usize>,
    pub intern: HashSet<usize>,
    pub ws: HashSet<usize>,
}

pub struct Root {
    pub common: Common,
    pub module: TokenStream,
    pub core: TokenStream,
}

pub struct Common {
    pub module: TokenStream,
    pub intern: TokenStream,
    pub memoize: TokenStream,
    pub packrat: TokenStream,
    pub stream: TokenStream,
}

pub fn s2c(s: &str) -> char {
    if s.starts_with("\\u{") && s.ends_with('}') {
        let hex = &s[3..s.len() - 1];
        let Ok(x) = u32::from_str_radix(hex, 16) else {
            unreachable!()
        };
        let Some(c) = char::from_u32(x) else {
            unreachable!()
        };
        return c;
    }

    if s.starts_with("\\") && s.len() == 2 {
        return match &s[1..] {
            "'" => '\'',
            "\"" => '"',
            "[" => '[',
            "]" => ']',
            "n" => '\n',
            "t" => '\t',
            "r" => '\r',
            "\\" => '\\',
            _ => unreachable!(),
        };
    }

    let mut chars = s.chars();
    let c = chars.next().unwrap();
    if chars.next().is_none() {
        c
    } else {
        unreachable!();
    }
}
