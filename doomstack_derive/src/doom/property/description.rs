use proc_macro2::TokenTree;

use crate::doom::Property;

impl Property {
    pub(in crate::doom::property) fn description(_body: Vec<TokenTree>) -> Property {
        todo!()
    }
}
