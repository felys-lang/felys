mod helper;

use crate::helper::*;
use proc_macro::TokenStream;

/// Cache the result of this sub-expression
///
/// Put `#[daybreak::memoize(AST)]` on top of the method,
/// where `AST` is the variant name in `Self::CT` and `Self::CR`.
#[proc_macro_attribute]
pub fn memoize(_: TokenStream, body: TokenStream) -> TokenStream {
    memoize_helper(body)
}

/// Allow left-recursion in this sub-expression, caching required
///
/// Put `#[daybreak::lecursion(AST)]` on top of the method,
/// where `AST` is the variant name in `Self::CT` and `Self::CR`.
#[proc_macro_attribute]
pub fn lecursion(_: TokenStream, body: TokenStream) -> TokenStream {
    lecursion_helper(body)
}

#[proc_macro_derive(Cache)]
pub fn cache(body: TokenStream) -> TokenStream {
    cache_helper(body)
}