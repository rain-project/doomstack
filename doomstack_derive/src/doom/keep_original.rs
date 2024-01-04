use proc_macro2::TokenTree;

#[allow(dead_code)]
pub(crate) struct KeepOriginal {
    pub condition: Option<Vec<TokenTree>>,
}
