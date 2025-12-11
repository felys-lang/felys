use crate::ast::{Action, Alter, Assignment, Atom, Expect, Item, Lookahead, Message, Nested, Rule};
use crate::builder::common::{Builder, Root, Template};
use crate::builder::dfa::common::Automaton;
use crate::philia093::Intern;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::parse_str;

impl Builder {
    pub fn codegen(&self) -> Root {
        let mut memo = Vec::new();
        let mut methods = Vec::new();

        for (id, template) in &self.order {
            let name = format_ident!("{}", self.intern.get(id).unwrap());
            let (ty, constant, body) = match template {
                Template::Rule => self.rule(id),
                Template::Lang => self.lang(id),
            };
            let method = quote! {
                pub fn #name(&mut self) -> Option<#ty> {
                    if self.__snapshot.is_some() {
                        return None;
                    }
                    #constant
                    #body
                }
            };
            methods.push(method);
            if self.tags.memo.contains(id) || self.tags.left.contains(id) {
                memo.push((name.to_token_stream(), ty));
            }
        }

        let import = self
            .import
            .as_ref()
            .map(|x| x.parse(&self.intern))
            .unwrap_or_default();
        let core = quote! {
            #import

            #[allow(clippy::double_parens, clippy::let_unit_value, clippy::clone_on_copy)]
            #[allow(non_snake_case, unused)]
            impl super::PhiLia093 {
                #(#methods)*
            }
        };

        self.template(core, memo)
    }

    fn rule(&self, id: &usize) -> (TokenStream, TokenStream, TokenStream) {
        let (ty, rule) = self.rules.get(id).unwrap();
        let name = format_ident!("{}", self.intern.get(id).unwrap());
        let ty = ty
            .as_ref()
            .map(|x| x.parse(&self.intern))
            .unwrap_or_else(|| quote! { () });
        let mut body = quote! { self.__peg(RULES) };

        if self.tags.left.contains(id) {
            body = quote! {
                let start = self.__stream.cursor;
                if let Some((end, cache)) = self.__memo.#name.get(&start) {
                    self.__stream.cursor = end.to_owned();
                    return cache.clone();
                }

                let mut result = None;
                let mut end = start;
                loop {
                    let cache = result.clone();
                    self.__memo.#name.insert(start, (end, cache));
                    let temp = #body;
                    if end < self.__stream.cursor {
                        result = temp;
                        end = self.__stream.cursor;
                        self.__stream.cursor = start;
                    } else {
                        self.__stream.cursor = end;
                        break;
                    }
                }

                let cache = result.clone();
                self.__memo.#name.insert(start, (end, cache));
                result
            }
        } else if self.tags.memo.contains(id) {
            body = quote! {
                let start = self.__stream.cursor;
                if let Some((end, cache)) = self.__memo.#name.get(&start) {
                    self.__stream.cursor = end.to_owned();
                    return cache.clone();
                }

                let result = #body;

                let end = self.__stream.cursor;
                let cache = result.clone();
                self.__memo.#name.insert(start, (end, cache));
                result
            }
        }

        let rules = rule.codegen(&self.intern);
        let size = rule.0.len();
        let constant = quote! {
            const RULES: super::R<#ty, #size> = #rules;
        };
        (ty, constant, body)
    }

    fn lang(&self, id: &usize) -> (TokenStream, TokenStream, TokenStream) {
        let language = self.langs.get(id).unwrap();
        let name = format_ident!("{}", self.intern.get(id).unwrap());
        let mut ty = quote! { usize };
        let mut body = quote! {
            self.__stream.dfa(transition, ACCEPTANCE)
        };

        if self.tags.memo.contains(id) {
            body = quote! {
                let start = self.__stream.cursor;
                if let Some(&(end, cache)) = self.__memo.#name.get(&start) {
                    self.__stream.cursor = end;
                    return cache;
                }

                let result = #body.map(|s| self.__intern.id(s));

                let end = self.__stream.cursor;
                self.__memo.#name.insert(start, (end, result));
                result
            };
        } else if self.tags.fast.contains(id) {
            body = quote! { #body.map(|_| ()) };
            ty = quote! { () };
        } else {
            body = quote! { #body.map(|s| self.__intern.id(s)) };
        }

        let constant = language.clone().pounded().build().codegen();
        (ty, constant, body)
    }
}

impl Action {
    pub fn parse(&self, intern: &Intern) -> TokenStream {
        let action = self
            .0
            .iter()
            .map(|x| x.restore(intern, ('{', '}')))
            .collect::<String>();
        parse_str::<TokenStream>(action.as_str()).unwrap()
    }
}

impl Nested {
    fn restore(&self, intern: &Intern, wrapper: (char, char)) -> String {
        match self {
            Nested::Inner(x) => x
                .iter()
                .map(|v| format!("{}{}{}", wrapper.0, v.restore(intern, wrapper), wrapper.1))
                .collect(),
            Nested::Segment(x) => intern.get(x).unwrap().to_string(),
        }
    }
}

impl Rule {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        let alters = self.0.iter().map(|x| x.codegen(intern));
        quote! { [#(#alters),*] }
    }
}

impl Alter {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        let items = self.assignments.iter().map(|x| x.codegen(intern));
        let product = if let Some(action) = &self.action {
            action.parse(intern)
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
                quote! { x.__memo.clean(); }
            }
            Assignment::Eof => {
                quote! { x.__eof()?; }
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
            Item::Eager(x, m) => {
                let atom = x.codegen(intern);
                let msg = if let Some(m) = m {
                    m.msg(intern)
                } else {
                    x.msg(intern)
                };
                quote! {
                    match #atom {
                        Some(value) => value,
                        None => return x.__error(#msg),
                    }
                }
            }
            Item::Repetition(x) => {
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
            Item::Name(x) => {
                let atom = x.codegen(intern);
                quote! { #atom? }
            }
        }
    }
}

impl Message {
    fn msg(&self, intern: &Intern) -> String {
        self.0
            .iter()
            .map(|x| x.restore(intern, ('(', ')')))
            .collect()
    }
}

impl Atom {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        match self {
            Atom::Name(x) => {
                let name = format_ident!("{}", intern.get(x).unwrap());
                quote! { x.#name() }
            }
            Atom::Expect(x) => x.codegen(intern),
            Atom::Nested(x) => {
                let rule = x.codegen(intern);
                quote! { x.__peg(#rule) }
            }
        }
    }

    fn msg(&self, intern: &Intern) -> String {
        match self {
            Atom::Name(x) => format!("<{}>", intern.get(x).unwrap()),
            Atom::Expect(x) => x.msg(intern).to_string(),
            Atom::Nested(_) => "???".to_string(),
        }
    }
}

impl Expect {
    fn codegen(&self, intern: &Intern) -> TokenStream {
        let expect = match self {
            Expect::Once(x) | Expect::Keyword(x) => x.squeeze(intern),
        };
        quote! { x.__expect(#expect) }
    }

    fn msg(&self, intern: &Intern) -> String {
        match self {
            Expect::Once(x) => format!("'{}'", x.squeeze(intern)),
            Expect::Keyword(x) => format!("\"{}\"", x.squeeze(intern)),
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
