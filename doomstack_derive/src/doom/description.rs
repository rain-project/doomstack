use proc_macro2::TokenTree;
use syn::LitStr;

pub(crate) enum Description {
    Static {
        description: LitStr,
    },
    Owned {
        format: LitStr,
        arguments: Vec<TokenTree>,
    },
}
