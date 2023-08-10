use crate::doom::{
    messages::{errors::*, helps::*},
    Fields, Settings,
};
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{Diagnostic, Level};
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
            } => Derive::derive_struct(identifier, settings, fields),

            Derive::Enum { .. } => {
                todo!()
            }
        }
    }
}

mod derive_struct;
mod parse_enum;
mod parse_struct;
