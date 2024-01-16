use proc_macro2::TokenTree;

pub(crate) struct KeepOriginal {
    pub condition: Option<Vec<TokenTree>>,
}
