use crate::doom::{
    messages::{errors::*, helps::*},
    Fields, Settings,
};
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{Diagnostic, Level};
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) enum Derive {
    Struct {
        identifier: Ident,
        settings: Settings,
        fields: Fields,
    },
    Enum {
        identifier: Ident,
        variants: Vec<Variant>,
    },
}

pub(crate) struct Variant {
    identifier: Ident,
    settings: Settings,
    fields: Fields,
}

impl Derive {
    pub fn parse(input: &DeriveInput) -> Self {
        let identifier = input.ident.clone();

        match &input.data {
            Data::Struct(data) => Derive::parse_struct(identifier, input, data),
            Data::Enum(data) => Derive::parse_enum(identifier, data),

            Data::Union(data) => {
                Diagnostic::spanned(
                    data.union_token.span,
                    Level::Error,
                    UNION_UNDERIVABLE.to_string(),
                )
                .help(DERIVABLES.to_string())
                .abort();
            }
        }
    }

    pub fn derive(&self) -> TokenStream {
        match self {
            Derive::Struct {
                identifier,
                settings,
                fields,
            } => {
                let doom = Derive::derive_struct_doom(identifier, settings, fields);
                let wraps = Derive::derive_struct_wraps(identifier, settings, fields);

                quote! {
                    #doom
                    #(#wraps)*
                }
            }

            Derive::Enum {
                identifier,
                variants,
            } => {
                let doom = Derive::derive_enum_doom(identifier, variants);
                let wraps = Derive::derive_enum_wraps(identifier, variants);

                quote! {
                    #doom
                    #(#wraps)*
                }
            }
        }
    }
}

mod derive_enum;
mod derive_struct;
mod derive_wrap;
mod parse_enum;
mod parse_struct;
