use crate::doom::{
    messages::{errors::*, helps::*},
    Description, Wrap,
};
use proc_macro_error::{Diagnostic, Level};

#[allow(dead_code)]
pub(crate) enum Attribute {
    Description(Description),
    Wrap(Wrap),
}

impl Attribute {
    pub fn parse(attribute: &syn::Attribute) -> Option<Self> {
        let (kind, body) = Attribute::parse_parts(attribute)?;

        let attribute = match kind.to_string().as_str() {
            "description" => Attribute::Description(Attribute::parse_description(body)),
            "wrap" => Attribute::Wrap(Attribute::parse_wrap(body)),
            _ => Diagnostic::spanned(kind.span(), Level::Error, UNEXPECTED_KIND.to_string())
                .help(AVAILABLE_KINDS.to_string())
                .abort(),
        };

        Some(attribute)
    }
}

mod parse_description;
mod parse_parts;
mod parse_wrap;
