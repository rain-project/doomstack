use crate::doom::{
    derive::Variant,
    messages::{errors::*, helps::*},
    Attribute, Derive, Fields, Settings,
};
use proc_macro2::Ident;
use proc_macro_error::{Diagnostic, Level};
use syn::DataEnum;

impl Derive {
    /// Parses an enum into a [`Derive`].
    ///
    /// Any error results in a graceful abort, indicating the problem with a meaningful message.
    pub(in crate::doom::derive) fn parse_enum(identifier: Ident, data: &DataEnum) -> Derive {
        // Parse each element of `data.variants` into a `Variant` (the mechanism to parse each
        // variant is similar to that implemented in `Derive::parse_struct` to parse a struct)

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

        // `variants` must not be empty (`Doom` cannot be derived on variant-less enums)

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
