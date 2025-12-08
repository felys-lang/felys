use crate::builder::common::{Builder, Common, Root};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

impl Builder {
    pub fn template(&self, core: TokenStream, memo: Vec<(TokenStream, TokenStream)>) -> Root {
        let import = self
            .import
            .as_ref()
            .map(|x| x.parse(&self.intern))
            .unwrap_or_default();
        let keywords = self.keywords.iter().map(|x| x.to_token_stream()).collect();

        Root {
            common: Common {
                module: quote! {
                    mod intern;
                    mod memoize;
                    mod packrat;
                    mod stream;

                    #[allow(unused)]
                    pub use intern::*;

                    #[allow(unused)]
                    pub use memoize::*;

                    #[allow(unused)]
                    pub use packrat::*;

                    #[allow(unused)]
                    pub use stream::*;
                },
                intern: intern(),
                memoize: memoize(import, memo),
                packrat: packrat(keywords),
                stream: stream(),
            },
            module: quote! {
                mod common;
                mod helper;
                mod core;

                #[allow(unused)]
                pub use helper::*;

                #[allow(unused)]
                pub use common::*;
            },
            core,
        }
    }
}

fn intern() -> TokenStream {
    quote! {
        use std::collections::HashMap;
        use std::rc::Rc;

        #[allow(unused)]
        #[derive(Default)]
        pub struct Intern {
            data: HashMap<Rc<str>, usize>,
            fast: Vec<Rc<str>>,
        }

        #[allow(unused)]
        impl Intern {
            pub fn id(&mut self, s: &str) -> usize {
                if let Some(&id) = self.data.get(s) {
                    id
                } else {
                    let key = Rc::<str>::from(s);
                    let id = self.fast.len();
                    self.fast.push(key.clone());
                    self.data.insert(key, id);
                    id
                }
            }

            pub fn get(&self, id: &usize) -> Option<&str> {
                let string = self.fast.get(*id)?;
                Some(&(**string))
            }
        }
    }
}

fn memoize(import: TokenStream, memo: Vec<(TokenStream, TokenStream)>) -> TokenStream {
    let names = memo.iter().map(|(name, _)| {
        quote! {
            self.#name.clear();
        }
    });
    let memo = memo.iter().map(|(name, ty)| {
        quote! {
            pub #name: HashMap<usize, (usize, Option<#ty>)>,
        }
    });
    quote! {
        use std::collections::HashMap;
        #import

        #[allow(non_snake_case, unused)]
        #[derive(Default)]
        pub struct Memo {
            #(#memo)*
        }

        #[allow(unused)]
        impl Memo {
            pub fn clean(&mut self) {
                #(#names)*
            }
        }
    }
}

fn packrat(keywords: Vec<TokenStream>) -> TokenStream {
    quote! {
        use std::collections::HashSet;

        #[allow(unused)]
        pub struct Packrat {
            pub __intern: super::Intern,
            pub __memo: super::Memo,
            pub __stream: super::Stream,
            pub __keywords: HashSet<&'static str>,
            pub __snapshot: Option<(usize, &'static str)>,
        }

        impl From<String> for Packrat {
            fn from(value: String) -> Self {
                Self {
                    __intern: super::Intern::default(),
                    __memo: super::Memo::default(),
                    __stream: super::Stream::from(value),
                    __keywords: HashSet::from([#(#keywords),*]),
                    __snapshot: None,
                }
            }
        }

        pub type Rules<T, const S: usize> = [fn(&mut Packrat) -> Option<T>; S];

        #[allow(unused)]
        impl Packrat {
            pub fn __expect(&mut self, s: &'static str) -> Option<&'static str> {
                if self.__snapshot.is_some() {
                    return None;
                }
                let start = self.__stream.cursor;
                let result = s
                    .chars()
                    .all(|c| self.__stream.next() == Some(c))
                    .then_some(s);
                if result.is_none() {
                    self.__stream.cursor = start;
                }
                result
            }

            pub fn __attempt<T>(&mut self, f: fn(&mut Packrat) -> Option<T>) -> Option<T> {
                let start = self.__stream.cursor;
                let result = f(self);
                if result.is_none() {
                    self.__stream.cursor = start;
                }
                result
            }

            pub fn __peg<T, const S: usize>(&mut self, rules: Rules<T, S>) -> Option<T> {
                rules.iter().filter_map(|rule| self.__attempt(*rule)).next()
            }

            pub fn __error<T>(&mut self, msg: &'static str) -> Option<T> {
                if self.__snapshot.is_some() {
                    return None;
                }
                let cursor = self.__stream.cursor;
                self.__snapshot = Some((cursor, msg));
                None
            }

            pub fn __lookahead<T>(&mut self, f: fn(&mut Packrat) -> Option<T>, behavior: bool) -> Option<()> {
                let start = self.__stream.cursor;
                let snapshot = self.__snapshot;
                let result = f(self);
                self.__stream.cursor = start;
                self.__snapshot = snapshot;
                if result.is_some() == behavior {
                    Some(())
                } else {
                    None
                }
            }
        }
    }
}

fn stream() -> TokenStream {
    quote! {
        #[allow(unused)]
        pub struct Stream {
            pub data: String,
            pub cursor: usize,
        }

        impl From<String> for Stream {
            fn from(value: String) -> Self {
                Self {
                    data: value,
                    cursor: 0,
                }
            }
        }

        impl Iterator for Stream {
            type Item = char;
            fn next(&mut self) -> Option<Self::Item> {
                let remaining = &self.data[self.cursor..];
                let ch = remaining.chars().next()?;
                self.cursor += ch.len_utf8();
                Some(ch)
            }
        }

        #[allow(unused)]
        impl Stream {
            pub fn dfa<const S: usize>(
                &mut self,
                transition: fn(usize, char) -> Option<usize>,
                acceptance: [bool; S]
            ) -> Option<&str> {
                let start = self.cursor;
                let mut end = start;
                let mut s = 0usize;
                while let Some(c) = self.next() {
                    s = match transition(s, c) {
                        Some(s) => s,
                        None => break,
                    };
                    end = self.cursor;
                }
                if acceptance[s] {
                    self.cursor = end;
                    Some(&self.data[start..end])
                } else {
                    self.cursor = start;
                    None
                }
            }

            pub fn peek(&mut self) -> Option<char> {
                self.data[self.cursor..].chars().next()
            }
        }
    }
}
