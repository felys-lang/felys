use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum, ItemFn, ReturnType};

pub fn memoize_helper(body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemFn);

    let vis = &body.vis;
    let sig = &body.sig;
    let output = match &body.sig.output {
        ReturnType::Default => panic!(),
        ReturnType::Type(_, output) => output
    };
    let block = &body.block;

    quote!(
        #vis #sig {
            let pos = self.stream.cursor;
            let mode = self.stream.strict;
            let sig = stringify!(#sig);
            if let Some(memo) = self.memo.get(pos, mode, sig) {
                let (end, res) = memo;
                self.stream.cursor = end;
                return res.into()
            }
            let result = || -> #output #block();
            let end = self.stream.cursor;
            let res = Self::CR::from(result.clone());
            self.memo.insert(pos, mode, sig, end, res);
            result
        }
    ).into()
}

pub fn lecursion_helper(body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemFn);

    let vis = &body.vis;
    let sig = &body.sig;
    let output = match &body.sig.output {
        ReturnType::Default => panic!(),
        ReturnType::Type(_, output) => output
    };
    let block = &body.block;

    quote!(
        #[::packrat::memoize]
        #vis #sig {
            let pos = self.stream.cursor;
            let none: #output = None;
            let mut res = Self::CR::from(none);
            let mut end = pos;
            loop {
                let mode = self.stream.strict;
                let sig = stringify!(#sig);
                self.memo.insert(pos, mode, sig, end, res.clone());
                let result = || -> #output #block();
                if end < self.stream.cursor {
                    res = Self::CR::from(result);
                    end = self.stream.cursor;
                    self.stream.cursor = pos;
                } else {
                    self.stream.cursor = end;
                    break res.into();
                }
            }
        }
    ).into()
}

pub fn cache_helper(body: TokenStream) -> TokenStream {
    let body = parse_macro_input!(body as ItemEnum);

    let cr = &body.ident;

    let all = body.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let option = match &variant.fields {
            Fields::Unnamed(x) => x.unnamed.first().unwrap(),
            _ => panic!()
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