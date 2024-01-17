use crate::doom::{
    attribute::Spans,
    messages::{errors::*, helps::*},
    Attribute, Wrap,
};
use proc_macro2::{Group, TokenTree};
use proc_macro_error::{Diagnostic, Level};
use std::collections::VecDeque;

impl Attribute {
    /// Parses the body of a `wrap` attribute into a [`Wrap`].
    ///
    /// Inputs the (optional) `body` of a `#[doom(wrap(body))]` attribute, as parsed by
    /// [`Attribute::parse_parts`]. Expects `body` to be `Some(constructor)`, where `constructor` is
    /// an [`Ident`] (of a wrapping constructor). Returns a [`Wrap`].
    ///
    /// [`Ident`]: struct@syn::Ident
    pub(in crate::doom::attribute) fn parse_wrap(body: Option<Group>, spans: &Spans) -> Wrap {
        // `body` must be `Some`

        let Some(body) = body else {
            Diagnostic::spanned(spans.kind, Level::Error, MISSING_WRAP_BODY.to_string())
                .help(WRAP_SYNTAX.to_string())
                .abort();
        };

        let mut tokens = body.stream().into_iter().collect::<VecDeque<_>>();

        // `body` must contain exactly one `Ident`

        let Some(constructor) = tokens.pop_front() else {
            // `body` is empty: `constructor` cannot be parsed

            Diagnostic::spanned(
                body.span(),
                Level::Error,
                MISSING_WRAPPING_CONSTRUCTOR.to_string(),
            )
            .help(WRAP_SYNTAX.to_string())
            .abort();
        };

        if let Some(token) = tokens.pop_front() {
            // Unexpected `token` after `constructor`: `body` is malformed

            Diagnostic::spanned(
                token.span(),
                Level::Error,
                UNEXPECTED_WRAP_TOKEN.to_string(),
            )
            .help(WRAP_SYNTAX.to_string())
            .abort();
        }

        let TokenTree::Ident(constructor) = constructor else {
            // `constructor` is not an `Ident`

            Diagnostic::spanned(
                constructor.span(),
                Level::Error,
                UNEXPECTED_WRAP_TOKEN.to_string(),
            )
            .help(WRAP_SYNTAX.to_string())
            .abort();
        };

        Wrap { constructor }
    }
}
