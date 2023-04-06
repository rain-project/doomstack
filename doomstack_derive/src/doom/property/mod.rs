use crate::doom::property::messages::{errors::*, helps::*};
use proc_macro2::TokenTree;
use proc_macro_error::{Diagnostic, Level};
use syn::{Attribute, Ident, LitStr};

#[allow(dead_code)]
pub(crate) enum Property {
    StaticDescription {
        description: LitStr,
    },
    OwnedDescription {
        format: LitStr,
        arguments: Vec<TokenTree>,
    },
    Wrap {
        constructor: Ident,
    },
}

impl Property {
    pub fn parse(attribute: &Attribute) -> Option<Self> {
        let (kind, body) = Property::parse_parts(attribute)?;

        let property = match kind.to_string().as_str() {
            "description" => Property::parse_description(body),
            "wrap" => Property::parse_wrap(body),
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
