mod helper;

use crate::helper::*;
use proc_macro::TokenStream;

/// Cache the result of this sub-expression
#[proc_macro_attribute]
pub fn memoize(_: TokenStream, body: TokenStream) -> TokenStream {
    memoize_helper(body)
}

/// Allow left-recursion in this sub-expression, caching required
#[proc_macro_attribute]
pub fn lecursion(_: TokenStream, body: TokenStream) -> TokenStream {
    lecursion_helper(body)
}

/// Derive `From<CR> for Option<T>` and the vice versa
#[proc_macro_derive(Cache)]
pub fn cache(body: TokenStream) -> TokenStream {
    cache_helper(body)
}