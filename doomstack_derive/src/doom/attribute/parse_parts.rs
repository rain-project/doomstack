use crate::doom::{
    messages::{errors::*, helps::*},
    Attribute,
};
use proc_macro2::{Delimiter, Group, TokenTree};
use proc_macro_error::{Diagnostic, Level};
use syn::{Ident, Meta};

impl Attribute {
    /// Parses a [`syn::Attribute`] to extract an [`Attribute`]'s kind and body.
    ///
    /// Returns `Some((kind, body))` if `attribute` is in the form ```#[doom(kind(body))]```,
    /// where `kind` is an [`Ident`] and `body` is a  sequence of [`TokenTree`]s; returns
    /// `None` otherwise.
    ///
    /// [`Ident`]: struct@syn::Ident
    pub(in crate::doom::attribute) fn parse_parts(
        attribute: &syn::Attribute,
    ) -> Option<(Ident, Group)> {
        // Return `None` if `attribute` is not in the form `#[doom(...)]`

        let Meta::List(meta) = &attribute.meta else {
            // `attribute` is not in the form `#[scope(...)]`
            return None;
        };

        let scope = if meta.path.segments.len() == 1 {
            &meta.path.segments[0]
        } else {
            // `attribute` is in the form `#[scope(...)]`, but `scope`
            // is a multi-segment path (e.g., `a::b::c`)
            return None;
        };

        if scope.ident != "doom" {
            // `attribute` is in the form `#[scope(...)]`, but `scope` is not `doom`
            return None;
        }

        // `attribute` is in the form `#[doom(...)]`: every error from this point
        // on should result in an abort, as a failure to parse would mean that
        // `attribute` is malformed

        let mut tokens = meta.tokens.clone().into_iter().collect::<Vec<_>>();

        // Note: `attribute` is in the form `#[doom(tokens)]`, i.e., `tokens`
        // stores the inner tokens of `attribute` (as a `Vec<TokenTree>`)

        // Abort if `tokens` is not in the form `kind(body)`

        // For `tokens` to be in the form `kind(body)`, it must contain two
        // elements: a `kind` `Ident` and a parenthesized block with the `body`

        if tokens.len() < 2 {
            Diagnostic::spanned(
                attribute.bracket_token.span.close(),
                Level::Error,
                INCOMPLETE_ATTRIBUTE.to_string(),
            )
            .help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        }

        if tokens.len() > 2 {
            Diagnostic::spanned(tokens[2].span(), Level::Error, UNEXPECTED_TOKEN.to_string())
                .help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
                .abort();
        }

        let kind = tokens.remove(0);
        let body = tokens.remove(0);

        // The first element of `attribute_tokens` must be an `Ident`

        let TokenTree::Ident(kind) = kind else {
            Diagnostic::spanned(
                kind.span(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string()
            ).help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        };

        // The second element of `attribute_tokens` must be a parenthesized block with the body

        let TokenTree::Group(body) = body else {
            Diagnostic::spanned(
                body.span(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string()
            ).help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        };

        if body.delimiter() != Delimiter::Parenthesis {
            Diagnostic::spanned(body.span_open(), Level::Error, UNEXPECTED_TOKEN.to_string())
                .help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
                .abort();
        }

        // All checks successful: return `kind` and `body`

        Some((kind, body))
    }
}
