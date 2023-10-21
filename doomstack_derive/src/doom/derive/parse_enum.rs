use crate::doom::{derive::Variant, Attribute, Derive, Fields, Settings};
use proc_macro2::Ident;
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
            .collect();

        Derive::Enum {
            identifier,
            variants,
        }
    }
}
