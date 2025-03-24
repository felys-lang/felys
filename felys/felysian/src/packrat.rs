use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum, ItemFn, ReturnType};

pub fn memoize_helper(body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemFn);

    let vis = &body.vis;
    let sig = &body.sig;
    let id = &sig.ident;
    let output = match &body.sig.output {
        ReturnType::Default => panic!(),
        ReturnType::Type(_, output) => output,
    };
    let block = &body.block;

    quote!(
        #vis #sig {
            let cur = self.stream.cursor;
            let s = self.stream.strict;
            let id = stringify!(#id);
            if let Some(memo) = self.memo.get(cur, s, id) {
                let (end, res) = memo;
                self.stream.cursor = end;
                return res.into()
            }
            let result = || -> #output #block();
            let end = self.stream.cursor;
            let res = result.clone().into();
            self.memo.insert(cur, s, id, end, res);
            result
        }
    )
    .into()
}

pub fn lecursion_helper(body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemFn);

    let vis = &body.vis;
    let sig = &body.sig;
    let id = &sig.ident;
    let output = match &body.sig.output {
        ReturnType::Default => panic!(),
        ReturnType::Type(_, output) => output,
    };
    let block = &body.block;

    quote!(
        #vis #sig {
            let cur = self.stream.cursor;
            let s = self.stream.strict;
            let id = stringify!(#id);
            if let Some(memo) = self.memo.get(cur, s, id) {
                let (end, res) = memo;
                self.stream.cursor = end;
                return res.into()
            }
            let result = || -> #output {
                let cur = self.stream.cursor;
                let mut res = None;
                let mut end = cur;
                loop {
                    let s = self.stream.strict;
                    let id = stringify!(#id);
                    self.memo.insert(cur, s, id, end, res.clone().into());
                    let result = || -> #output #block();
                    if end < self.stream.cursor {
                        res = result.into();
                        end = self.stream.cursor;
                        self.stream.cursor = cur;
                    } else {
                        self.stream.cursor = end;
                        break res.into();
                    }
                }
            }();
            let end = self.stream.cursor;
            let res = result.clone().into();
            self.memo.insert(cur, s, id, end, res);
            result
        }
    )
    .into()
}

pub fn cachable_helper(body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemEnum);

    let cr = &body.ident;

    let all = body.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let option = match &variant.fields {
            Fields::Unnamed(x) => x.unnamed.first().unwrap(),
            _ => panic!(),
        };

        quote!(
            impl From<#option> for #cr {
                fn from(value: #option) -> Self {
                    Self::#ident(value)
                }
            }

            impl From<#cr> for #option {
                fn from(value: #cr) -> Self {
                    match value {
                        #cr::#ident(inner) => inner,
                        _ => panic!()
                    }
                }
            }
        )
    });

    quote!(#(#all)*).into()
}
