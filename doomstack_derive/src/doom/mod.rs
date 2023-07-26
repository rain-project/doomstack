// Modules

mod description;
mod property;

// Interface

use crate::doom::property::Property;
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
