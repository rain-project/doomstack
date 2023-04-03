use crate::doom::property::messages::{errors::*, helps::*};
use proc_macro_error::{Diagnostic, Level};
use syn::{Attribute, Ident, LitStr};

#[allow(dead_code)]
pub(crate) enum Property {
    StaticDescription {
        description: LitStr,
    },
    OwnedDescription {
        description: LitStr,
        arguments: Vec<Ident>,
    },
    Wrap {
        constructor: Ident,
    },
}

impl Property {
    pub fn parse(attribute: &Attribute) -> Option<Self> {
        let (kind, body) = Property::tokens(attribute)?;

        let property = match kind.to_string().as_str() {
            "description" => Property::description(body),
            "wrap" => Property::wrap(body),
            _ => Diagnostic::spanned(kind.span(), Level::Error, UNEXPECTED_KIND.to_string())
                .help(AVAILABLE_KINDS.to_string())
                .abort(),
        };

        Some(property)
    }
}

mod description;
mod messages;
mod tokens;
mod wrap;
