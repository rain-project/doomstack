use crate::doom::{
    derive::Variant,
    messages::{errors::*, helps::*},
    Attribute, Derive, Fields, Settings,
};
use proc_macro2::Ident;
use proc_macro_error::{Diagnostic, Level};
use syn::DataEnum;

impl Derive {
    pub(in crate::doom::derive) fn parse_enum(identifier: Ident, data: &DataEnum) -> Derive {
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
            .collect::<Vec<_>>();

        if variants.is_empty() {
            Diagnostic::spanned(
                identifier.span(),
                Level::Error,
                ENUM_WITHOUT_VARIANTS.to_string(),
            )
            .help(ENUM_NEEDS_VARIANTS.to_string())
            .abort();
        }

        Derive::Enum {
            identifier,
            variants,
        }
    }
}
