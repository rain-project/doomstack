// Modules

mod attribute;
mod derive;
mod description;
mod fields;
mod keep_original;
mod messages;
mod setting;
mod settings;
mod wrap;

use attribute::Attribute;
use derive::Derive;
use description::Description;
use fields::Fields;
use keep_original::KeepOriginal;
use setting::Setting;
use settings::Settings;
use wrap::Wrap;

// Interface

use proc_macro::TokenStream;
use syn::DeriveInput;

/// Derives the `Doom` trait for the provided `input`.
pub(crate) fn doom(input: TokenStream) -> TokenStream {
    // Parse `input` into a `DeriveInput`
    let input: DeriveInput = syn::parse(input).unwrap();

    // Parse `input` into a `Derive`
    let derive = Derive::parse(&input);

    // Derive `derive` into a `TokenStream`
    derive.derive().into()
}
