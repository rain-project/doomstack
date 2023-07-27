use proc_macro2::TokenTree;
use syn::LitStr;

#[allow(dead_code)]
pub(crate) enum Description {
    Static {
        description: LitStr,
    },
    Owned {
        format: LitStr,
        arguments: Vec<TokenTree>,
    },
}
