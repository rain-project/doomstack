use crate::doom::{Attribute, Derive, Fields, Settings};
use proc_macro2::Ident;
use syn::{DataStruct, DeriveInput};

impl Derive {
    pub(in crate::doom::derive) fn parse_struct(
        identifier: Ident,
        input: &DeriveInput,
        data: &DataStruct,
    ) -> Derive {
        let attributes = input.attrs.iter().filter_map(Attribute::parse);
        let settings = Settings::from_attributes(attributes, identifier.span());
        let fields = Fields::parse(&data.fields);

        Derive::Struct {
            identifier,
            settings,
            fields,
        }
    }
}
