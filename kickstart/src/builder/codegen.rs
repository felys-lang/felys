use crate::ast::{Alter, Assignment, Atom, Item, Lookahead, Rule, Tag};
use crate::builder::common::{s2c, Builder, Root};
use crate::builder::dfa::common::{Automaton, Language};
use crate::parser::Intern;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::iter::once;
use syn::parse_str;

impl Builder {
    pub fn codegen(&self) -> Root {
        let mut memo = Vec::new();
        let mut methods = Vec::new();

        for id in &self.sequence {
            let (name, ty, body) = self.method(id);
            let method = quote! {
                pub fn #name(&mut self) -> Option<#ty> {
                    #body
                }
            };
            methods.push(method);
            if self.tags.memo.contains(id) || self.tags.left.contains(id) {
                memo.push((name, ty));
            }
        }

        let mut whitespace = self.tags.ws.iter();
        let trim = if let Some(first) = whitespace.next() {
            let mut node = self.languages.get(first).unwrap().clone();
            for name in whitespace {
                node = Language::Union(
                    node.into(),
                    self.languages.get(name).unwrap().clone().into(),
                );
            }
            let constants = node.pounded().build().codegen();
            quote! {
                if self.strict {
                    return;
                }
                #constants
                loop {
                    if self.dfa(transition, ACCEPTANCE).is_none() {
                        break;
                    }
                }
            }
        } else {
            quote! {}
        };

        let import = match &self.import {
            Some(x) => parse_str(self.intern.get(x).unwrap()).unwrap(),
            None => quote! {},
        };
        let parser = quote! {
            #import

            #[allow(clippy::double_parens, clippy::let_unit_value)]
            #[allow(non_snake_case, unused)]
            impl super::Packrat {
                #(#methods)*
            }

            #[allow(unused)]
            impl super::Stream {
                pub fn trim(&mut self) {
                    #trim
                }
            }
        };

        self.template(parser, memo)
    }

    fn method(&self, id: &usize) -> (TokenStream, TokenStream, TokenStream) {
        let name = format_ident!("{}", self.intern.get(id).unwrap());
        let (ty, constant, body) = if let Some((ty, rule)) = self.rules.get(id) {
            let ty = parse_str::<TokenStream>(self.intern.get(ty).unwrap()).unwrap();
            let body = if self.tags.token.contains(id) {
                quote! { self.__token(RULES) }
            } else {
                quote! { self.__rule(RULES) }
            };

            let body = if self.tags.left.contains(id) {
                quote! {
                    let start = self.stream.cursor;
                    let strict = self.stream.strict;
                    if let Some((end, cache)) = self.memo.#name.get(&(start, strict)) {
                        self.stream.cursor = end.to_owned();
                        return cache.clone();
                    }

                    let mut result = None;
                    let mut end = start;
                    loop {
                        let cache = result.clone();
                        self.memo.#name.insert((start, strict), (end, cache));
                        let temp = #body;
                        if end < self.stream.cursor {
                            result = temp;
                            end = self.stream.cursor;
                            self.stream.cursor = start;
                        } else {
                            self.stream.cursor = end;
                            break;
                        }
                    }

                    let cache = result.clone();
                    self.memo.#name.insert((start, strict), (end, cache));
                    result
                }
            } else if self.tags.memo.contains(id) {
                quote! {
                    let start = self.stream.cursor;
                    let strict = self.stream.strict;
                    if let Some((end, cache)) = self.memo.#name.get(&(start, strict)) {
                        self.stream.cursor = end.to_owned();
                        return cache.clone();
                    }

                    let result = #body;

                    let end = self.stream.cursor;
                    let cache = result.clone();
                    self.memo.#name.insert((start, strict), (end, cache));
                    result
                }
            } else {
                body
            };

            let rules = rule.codegen(&self.intern);
            let size = 1 + rule.more.len();
            let constant = quote! {
                const RULES: super::Rules<#ty, #size> = #rules;
            };
            (ty, constant, body)
        } else if let Some(language) = self.languages.get(id) {
            let ty = if self.tags.intern.contains(id) {
                quote! { usize }
            } else {
                quote! { () }
            };
            let body = quote! { self.stream.dfa(transition, ACCEPTANCE) };
            let body = if self.tags.intern.contains(id) {
                quote! { #body.map(|s| self.intern.id(s)) }
            } else {
                quote! { #body.and(Some(())) }
            };
            let body = if self.tags.memo.contains(id) {
                quote! {
                    let start = self.stream.cursor;
                    let strict = self.stream.strict;
                    if let Some(&(end, cache)) = self.memo.#name.get(&(start, strict)) {
                        self.stream.cursor = end;
                        return cache;
                    }

                    self.stream.trim();
                    let result = #body;

                    let end = self.stream.cursor;
                    self.memo.#name.insert((start, strict), (end, result));
                    result
                }
            } else {
                quote! {
                    self.stream.trim();
                    #body
                }
            };
            let constant = language.clone().pounded().build().codegen();
            (ty, constant, body)
        } else {
            panic!()
        };
        let body = quote! {
            if self.snapshot.is_some() {
                return None;
            }
            #constant
            #body
        };
        (name.to_token_stream(), ty, body)
    }
}

impl Rule {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        let first = once(self.first.codegen(intern));
        let more = self.more.iter().map(|x| x.codegen(intern));
        let alters = first.chain(more);
        quote! { [#(#alters),*] }
    }
}

impl Alter {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        let items = self.assignments.iter().map(|x| x.codegen(intern));
        let product = if let Some(action) = &self.action {
            let result = intern.get(action).unwrap();
            parse_str::<TokenStream>(result).unwrap()
        } else {
            let names = self.assignments.iter().filter_map(|x| {
                if let Assignment::Named(name, _) = x {
                    Some(format_ident!("{}", intern.get(name).unwrap()))
                } else {
                    None
                }
            });
            quote! { (#(#names),*) }
        };

        quote! {
            |x| {
                #(#items)*
                Some(#product)
            }
        }
    }
}

impl Assignment {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        match self {
            Assignment::Named(n, x) => {
                let name = format_ident!("{}", intern.get(n).unwrap());
                let item = x.codegen(intern);
                quote! { let #name = #item; }
            }
            Assignment::Lookahead(x) => {
                let lookahead = x.codegen(intern);
                quote! { let _ = #lookahead; }
            }
            Assignment::Anonymous(x) => {
                let item = x.codegen(intern);
                quote! { let _ = #item; }
            }
            Assignment::Clean => {
                quote! { x.memo.clean(); }
            }
        }
    }
}

impl Lookahead {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        match self {
            Lookahead::Positive(x) => {
                let atom = x.codegen(intern);
                quote! { x.__lookahead(|x| #atom, true)? }
            }
            Lookahead::Negative(x) => {
                let atom = x.codegen(intern);
                quote! { x.__lookahead(|x| #atom, false)? }
            }
        }
    }
}

impl Item {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        match self {
            Item::OnceOrMore(e, x) => {
                let atom = x.codegen(intern);
                let first = if e.to_owned() {
                    let msg = x.msg(intern);
                    quote! {
                        match #atom {
                            Some(value) => value,
                            None => return x.__error(#msg),
                        }
                    }
                } else {
                    quote! { #atom? }
                };
                quote! {
                    {
                        let first = #first;
                        let mut body = Vec::from([first]);
                        while let Some(data) = #atom {
                            body.push(data)
                        }
                        body
                    }
                }
            }
            Item::ZeroOrMore(x) => {
                let atom = x.codegen(intern);
                quote! {
                    {
                        let mut body = Vec::new();
                        while let Some(data) = #atom {
                            body.push(data)
                        }
                        body
                    }
                }
            }
            Item::Optional(x) => x.codegen(intern),
            Item::Name(e, x) => {
                let atom = x.codegen(intern);
                if e.to_owned() {
                    let msg = x.msg(intern);
                    quote! {
                        match #atom {
                            Some(value) => value,
                            None => return x.__error(#msg),
                        }
                    }
                } else {
                    quote! { #atom? }
                }
            }
        }
    }
}

impl Atom {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        match self {
            Atom::Name(x) => {
                let name = format_ident!("{}", intern.get(x).unwrap());
                quote! { x.#name() }
            }
            Atom::String(x) => {
                let string = x
                    .iter()
                    .map(|x| s2c(intern.get(x).unwrap()))
                    .collect::<String>();
                quote! { x.__expect(#string) }
            }
            Atom::Nested(deco, x) => {
                let rule = x.codegen(intern);
                if let Some(deco) = deco {
                    if let Some(Tag::Token) = deco.tags.first() {
                        quote! { x.__token(#rule) }
                    } else {
                        panic!("tag not supported")
                    }
                } else {
                    quote! { x.__rule(#rule) }
                }
            }
        }
    }

    fn msg(&self, intern: &Intern) -> String {
        match self {
            Atom::Name(x) => format!("<{}>", intern.get(x).unwrap()),
            Atom::String(x) => format!(
                "'{}'",
                x.iter()
                    .map(|x| s2c(intern.get(x).unwrap()))
                    .collect::<String>()
            ),
            Atom::Nested(_, _) => "???".to_string(),
        }
    }
}

impl Automaton {
    fn codegen(&self) -> TokenStream {
        let transition = self.transition.iter().map(|x| {
            let (s0, (s, e), s1) = x;
            quote! { (#s0, #s..=#e) => #s1, }
        });
        let acceptance = &self.acceptance;
        let size = acceptance.len();
        quote! {
            fn transition(s: usize, c: char) -> Option<usize> {
                let s = match (s, c as usize) {
                    #(#transition)*
                    _ => return None,
                };
                Some(s)
            }
            const ACCEPTANCE: [bool; #size] = [#(#acceptance),*];
        }
    }
}
