use crate::doom::{
    attribute::Spans,
    messages::{errors::*, helps::*},
    Attribute, Wrap,
};
use proc_macro2::{Group, TokenTree};
use proc_macro_error::{Diagnostic, Level};

impl Attribute {
    /// Parses the body of a `wrap` attribute into a [`Wrap`]
    ///
    /// Inputs the `body` of a `#[doom(wrap(body))]` attribute. Expects `body` to
    /// be `Some(body)`, where `body` is an [`Ident`] (of a wrapping constructor).
    /// Returns a [`Wrap`].
    pub(in crate::doom::attribute) fn parse_wrap(body: Option<Group>, spans: &Spans) -> Wrap {
        // `body` must be `Some`

        let Some(body) = body else {
            Diagnostic::spanned(spans.kind, Level::Error, MISSING_WRAP_BODY.to_string())
                .help(WRAP_STYLE.to_string())
                .abort();
        };

        let mut tokens = body.stream().into_iter().collect::<Vec<_>>();

        // `body` must contain exactly one `Ident`

        if tokens.is_empty() {
            Diagnostic::spanned(
                body.span(),
                Level::Error,
                MISSING_WRAPPING_CONSTRUCTOR.to_string(),
            )
            .help(WRAP_STYLE.to_string())
            .abort();
        }

        if tokens.len() > 1 {
            Diagnostic::spanned(
                tokens[1].span(),
                Level::Error,
                UNEXPECTED_WRAP_TOKEN.to_string(),
            )
            .help(WRAP_STYLE.to_string())
            .abort();
        }

        let constructor = tokens.remove(0);

        let TokenTree::Ident(constructor) = constructor else {
            Diagnostic::spanned(
                constructor.span(),
                Level::Error,
                UNEXPECTED_WRAP_TOKEN.to_string(),
            )
            .help(WRAP_STYLE.to_string())
            .abort();
        };

        Wrap { constructor }
    }
}
