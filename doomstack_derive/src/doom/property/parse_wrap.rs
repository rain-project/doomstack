use crate::doom::{
    messages::{errors::*, helps::*},
    Property, Wrap,
};
use proc_macro2::{Group, TokenTree};
use proc_macro_error::{Diagnostic, Level};

impl Property {
    /// Parses the body of a `wrap` attribute into a [`Wrap`]
    ///
    /// Inputs the `body` of a `#[doom(wrap(body))]` attribute. Expects `body`
    /// to be an [`Ident`] (of a wrapping constructor). Returns a [`Wrap`].
    pub(in crate::doom::property) fn parse_wrap(body: Group) -> Wrap {
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
