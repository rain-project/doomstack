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
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn doom(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();

    let Data::Enum(data) = derive_input.data else { todo!() };
    let variant = &data.variants[0];

    let attributes = variant.attrs.iter().filter_map(Attribute::parse);
    let _settings = Settings::from_attributes(attributes, variant.ident.span());

    quote!().into()
}
