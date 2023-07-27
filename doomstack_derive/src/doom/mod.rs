// Modules

mod description;
mod messages;
mod property;
mod wrap;

use description::Description;
use property::Property;
use wrap::Wrap;

// Interface

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn doom(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();

    let Data::Enum(mut data) = derive_input.data else { todo!() };

    let variant = data.variants[0].attrs.remove(0);
    let _property = Property::parse(&variant);

    quote!().into()
}
