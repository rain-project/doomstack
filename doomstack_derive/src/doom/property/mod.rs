use crate::doom::{
    property::messages::{errors::*, helps::*},
    Description, Wrap,
};
use proc_macro_error::{Diagnostic, Level};
use syn::Attribute;

#[allow(dead_code)]
pub(crate) enum Property {
    Description(Description),
    Wrap(Wrap),
}

impl Property {
    pub fn parse(attribute: &Attribute) -> Option<Self> {
        let (kind, body) = Property::parse_parts(attribute)?;

        let property = match kind.to_string().as_str() {
            "description" => Property::Description(Property::parse_description(body)),
            "wrap" => Property::Wrap(Property::parse_wrap(body)),
            _ => Diagnostic::spanned(kind.span(), Level::Error, UNEXPECTED_KIND.to_string())
                .help(AVAILABLE_KINDS.to_string())
                .abort(),
        };

        Some(property)
    }
}

mod messages;
mod parse_description;
mod parse_parts;
mod parse_wrap;
