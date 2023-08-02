use crate::doom::{
    messages::{errors::*, helps::*},
    Attribute, Fields, Settings,
};
use proc_macro2::Ident;
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
            Data::Struct(data) => {
                let attributes = input.attrs.iter().filter_map(Attribute::parse);
                let settings = Settings::from_attributes(attributes, identifier.span());
                let fields = Fields::parse(&data.fields);

                Derive::Struct {
                    identifier,
                    settings,
                    fields,
                }
            }

            Data::Enum(data) => {
                let variants = data
                    .variants
                    .iter()
                    .map(|variant| {
                        let identifier = variant.ident.clone();
                        let attributes = variant.attrs.iter().filter_map(Attribute::parse);
                        let settings = Settings::from_attributes(attributes, identifier.span());
                        let fields = Fields::parse(&variant.fields);

                        Variant {
                            identifier,
                            settings,
                            fields,
                        }
                    })
                    .collect();

                Derive::Enum {
                    identifier,
                    variants,
                }
            }

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
}
