use crate::doom::{
    attribute::Spans,
    messages::{errors::*, helps::*},
    Attribute, Description,
};
use proc_macro2::{Group, TokenTree};
use proc_macro_error::{Diagnostic, Level};
use std::collections::VecDeque;
use syn::Lit;

impl Attribute {
    /// Parses the body of a `description` attribute into a [`Description`].
    ///
    /// Inputs the (optional) `body` of a `#[doom(description(body))]` attribute, as parsed by
    /// [`Attribute::parse_parts`]. Expects `body` to be `Some(body)`, with `body` feedable into the
    /// macro [`format!`], i.e., a format string literal followed by zero or more arguments to
    /// format. Returns a [`Description`].
    ///
    /// [`Setting`]: crate::doom::Setting
    pub(in crate::doom::attribute) fn parse_description(
        body: Option<Group>,
        spans: &Spans,
    ) -> Description {
        // `body` must be `Some`

        let Some(body) = body else {
            Diagnostic::spanned(
                spans.kind,
                Level::Error,
                MISSING_DESCRIPTION_BODY.to_string(),
            )
            .help(DESCRIPTION_SYNTAX.to_string())
            .abort();
        };

        let mut tokens = body.stream().into_iter().collect::<VecDeque<_>>();

        // `body` must contain at least one format `LitStr`

        let Some(format) = tokens.pop_front() else {
            Diagnostic::spanned(
                body.span(),
                Level::Error,
                MISSING_DESCRIPTION_FORMAT.to_string(),
            )
            .help(DESCRIPTION_SYNTAX.to_string())
            .abort();
        };

        let TokenTree::Literal(format) = format else {
            Diagnostic::spanned(
                format.span(),
                Level::Error,
                UNEXPECTED_DESCRIPTION_TOKEN.to_string(),
            )
            .help(DESCRIPTION_SYNTAX.to_string())
            .abort();
        };

        let format = Lit::new(format);

        let Lit::Str(format) = format else {
            Diagnostic::spanned(
                format.span(),
                Level::Error,
                UNEXPECTED_DESCRIPTION_TOKEN.to_string(),
            )
            .help(DESCRIPTION_SYNTAX.to_string())
            .abort();
        };

        // Determine if `format` formats variables: if so, the `Description` is `Owned`,
        // otherwise it is `Static`

        // `format` formats variables if and only if it contains single brackets ('{') (double
        // brackets ("{{") are an escape sequence in `format!`, and are formatted as a literal '{'):
        // remove all double brackets ("{{"), then check if brackets ('{') are still present
        if format.value().replace("{{", "").contains('{') {
            // `format` formats variables: all remaining `tokens` are arguments to the `format!`
            // macro that `Doom::description` will use to produce the item's description.

            // Note: no further validation is necessary - `format` and `tokens` will be fed directly
            // into the `format!` macro, possibly resulting in errors: in that case, `format!` will
            // produce a meaningful error, referencing the correct `Span`s of `format` and `tokens`.

            Description::Owned {
                format,
                arguments: tokens.into(),
            }
        } else {
            // `format` does not format variables: there should be no `tokens`
            // remaining (this would result in orphan arguments to `format!`)

            if let Some(token) = tokens.pop_front() {
                Diagnostic::spanned(
                    token.span(),
                    Level::Error,
                    UNEXPECTED_DESCRIPTION_ARGUMENTS.to_string(),
                )
                .help(DESCRIPTION_SYNTAX.to_string())
                .abort();
            }

            Description::Static {
                description: format,
            }
        }
    }
}
