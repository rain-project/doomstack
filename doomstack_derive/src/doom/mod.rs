// Modules

mod attribute;
mod derive;
mod description;
mod fields;
mod messages;
mod setting;
mod settings;
mod wrap;

use attribute::Attribute;
use derive::Derive;
use description::Description;
use fields::Fields;
use setting::Setting;
use settings::Settings;
use wrap::Wrap;

// Interface

use proc_macro::TokenStream;
use syn::DeriveInput;

pub(crate) fn doom(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let derive = Derive::parse(&input);

    derive.derive().into()
}
