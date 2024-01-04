use proc_macro2::TokenTree;
use syn::LitStr;

// TODO: Remove all `#[allow(dead_code)]`
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
