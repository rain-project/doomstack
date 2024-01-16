use crate::doom::{
    messages::{errors::*, helps::*},
    Setting,
};
use proc_macro2::Span;
use proc_macro_error::{Diagnostic, Level};

pub(crate) struct Attribute {
    pub spans: Spans,
    pub setting: Setting,
}

pub(crate) struct Spans {
    pub kind: Span,
}

impl Attribute {
    pub fn parse(attribute: &syn::Attribute) -> Option<Self> {
        let (kind, body) = Attribute::parse_parts(attribute)?;

        let spans = Spans { kind: kind.span() };

        let setting = match kind.to_string().as_str() {
            "description" => Setting::Description(Attribute::parse_description(body, &spans)),
            "keep_original" => Setting::KeepOriginal(Attribute::parse_keep_original(body)),
            "wrap" => Setting::Wrap(Attribute::parse_wrap(body, &spans)),

            _ => Diagnostic::spanned(kind.span(), Level::Error, UNEXPECTED_KIND.to_string())
                .help(AVAILABLE_KINDS.to_string())
                .abort(),
        };

        Some(Attribute { spans, setting })
    }
}

mod parse_description;
mod parse_keep_original;
mod parse_parts;
mod parse_wrap;
