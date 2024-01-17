use crate::doom::{Attribute, Derive, Fields, Settings};
use proc_macro2::Ident;
use syn::DataStruct;

impl Derive {
    /// Parses a struct into a [`Derive`].
    ///
    /// Any error results in a graceful abort, indicating the problem with a meaningful message.
    pub(in crate::doom::derive) fn parse_struct(
        identifier: Ident,
        attributes: &[syn::Attribute],
        data: &DataStruct,
    ) -> Derive {
        let attributes = attributes.iter().filter_map(Attribute::parse);
        let settings = Settings::from_attributes(attributes, identifier.span());
        let fields = Fields::parse(&data.fields);

        Derive::Struct {
            identifier,
            settings,
            fields,
        }
    }
}
