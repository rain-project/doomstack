use crate::doom::{
    property::messages::{errors::*, helps::*},
    Property,
};
use proc_macro2::{Delimiter, TokenTree};
use proc_macro_error::{Diagnostic, Level};
use syn::{Attribute, Ident, Meta};

impl Property {
    /// Parses an [`Attribute`] to extract a [`Property`]'s kind and body.
    ///
    /// Returns `Some((kind, body))` if `attribute` is in the form ```#[doom(kind(body))]```,
    /// where `kind` is an [`Ident`] and `body` is a  sequence of [`TokenTree`]s; returns
    /// `None` otherwise.
    ///
    /// [`Ident`]: struct@syn::Ident
    pub(in crate::doom::property) fn tokens(
        attribute: &Attribute,
    ) -> Option<(Ident, Vec<TokenTree>)> {
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

        let tokens = meta.tokens.clone().into_iter().collect::<Vec<_>>();

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

        // The first element of `attribute_tokens` must be an `Ident`

        let TokenTree::Ident(kind) = tokens[0].clone() else {
            Diagnostic::spanned(
                tokens[0].span(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string()
            ).help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        };

        // The second element of `attribute_tokens` must be a parenthesized block with the body

        let TokenTree::Group(body_group) = &tokens[1] else {
            Diagnostic::spanned(
                tokens[1].span(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string()
            ).help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        };

        if body_group.delimiter() != Delimiter::Parenthesis {
            Diagnostic::spanned(
                body_group.span_open(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string(),
            )
            .help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        }

        let body = body_group.stream().into_iter().collect::<Vec<_>>();

        // All checks successful: return `kind` and `body`

        Some((kind, body))
    }
}
