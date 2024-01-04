use crate::doom::{
    attribute::Spans,
    messages::{errors::*, helps::*},
    Attribute, Description,
};
use proc_macro2::{Group, TokenTree};
use proc_macro_error::{Diagnostic, Level};
use syn::Lit;

impl Attribute {
    /// Parses the body of a `description` attribute into a [`Description`]
    ///
    /// Inputs the `body` of a `#[doom(description(body))]` attribute. Expects
    /// `body` to be `Some(body)`, with `body` feedable into [`format!`], i.e.,
    /// a format string literal followed by zero or more arguments to format.
    /// Returns a [`Description`].
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
            .help(DESCRIPTION_STYLE.to_string())
            .abort();
        };

        let mut tokens = body.stream().into_iter().collect::<Vec<_>>();

        // `body` must contain at least one format `LitStr`

        if tokens.is_empty() {
            Diagnostic::spanned(
                body.span(),
                Level::Error,
                MISSING_DESCRIPTION_FORMAT.to_string(),
            )
            .help(DESCRIPTION_STYLE.to_string())
            .abort();
        }

        let format = tokens.remove(0);

        let TokenTree::Literal(format) = format else {
            Diagnostic::spanned(
                format.span(),
                Level::Error,
                UNEXPECTED_DESCRIPTION_TOKEN.to_string(),
            )
            .help(DESCRIPTION_STYLE.to_string())
            .abort();
        };

        let format = Lit::new(format);

        let Lit::Str(format) = format else {
            Diagnostic::spanned(
                format.span(),
                Level::Error,
                UNEXPECTED_DESCRIPTION_TOKEN.to_string(),
            )
            .help(DESCRIPTION_STYLE.to_string())
            .abort();
        };

        // Determine if `format` formats variables: if so, the
        // `Description` is `Owned`, otherwise it is `Static`

        // `format` formats variables if and only if it contains
        // single brackets ('{') (double brackets ("{{") are an
        // escape sequence in `format!`, and are formatted as a
        // literal '{'): remove all double brackets ("{{"), then
        // check if brackets ('{') are still present
        if format.value().replace("{{", "").contains('{') {
            // `format` formats variables: all remaining `tokens` are
            // arguments to the `format!` macro that `Doom::description`
            // will use to produce the item's description

            // Note: no further validation is necessary - `format` and
            // `tokens` will be fed directly into the `format!` macro,
            // possibly resulting in errors: in that case, `format!`
            // will result in a meaningful error, referencing the
            // correct `Span`s of `format` and `tokens`

            Description::Owned {
                format,
                arguments: tokens,
            }
        } else {
            // `format` does not format variables: there should be no `tokens`
            // remaining (this would result in orphan arguments to `format!`)

            if !tokens.is_empty() {
                Diagnostic::spanned(
                    tokens[0].span(),
                    Level::Error,
                    UNEXPECTED_DESCRIPTION_ARGUMENTS.to_string(),
                )
                .help(DESCRIPTION_STYLE.to_string())
                .abort();
            }

            Description::Static {
                description: format,
            }
        }
    }
}
