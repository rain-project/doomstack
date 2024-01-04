use crate::doom::{
    messages::{errors::*, helps::*},
    Attribute,
};
use proc_macro2::{Delimiter, Group, TokenTree};
use proc_macro_error::{Diagnostic, Level};
use std::collections::VecDeque;
use syn::{Ident, Meta};

// TODO: Fix `Attribute::parse_parts` for `#[doom]` (it should produce an error, it doesn't)

impl Attribute {
    /// Parses a [`syn::Attribute`] to extract an [`Attribute`]'s kind and (optional) body.
    ///
    /// Returns:
    ///  - `Some((kind, Some(body)))` if `attribute` is in the form `#[doom(kind(body))]`,
    ///     where `kind` is an [`Ident`] and `body` is a  sequence of [`TokenTree`]s;
    ///  - `Some((kind, None))` if `attribute` is in the form `#[doom(kind)]`, where
    ///     `kind` is an [`Ident`];
    ///  - `None` otherwise.
    ///
    /// [`Ident`]: struct@syn::Ident
    pub(in crate::doom::attribute) fn parse_parts(
        attribute: &syn::Attribute,
    ) -> Option<(Ident, Option<Group>)> {
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

        let mut tokens = meta.tokens.clone().into_iter().collect::<VecDeque<_>>();

        // Note: `attribute` is in the form `#[doom(tokens)]`, i.e., `tokens`
        // stores the inner tokens of `attribute` (as a `VecDeque<TokenTree>`)

        // Abort if `tokens` is not in the form `kind` or `kind(body)`

        // For `tokens` to be in the form `kind` or `kind(body)`, it must contain
        // one or two elements: a `kind` `Ident` and an (optional) parenthesized
        // block with the `body`

        if tokens.is_empty() {
            Diagnostic::spanned(
                attribute.bracket_token.span.close(),
                Level::Error,
                EMPTY_ATTRIBUTE.to_string(),
            )
            .help(ATTRIBUTES_SYNTAX.to_string())
            .abort();
        }

        if tokens.len() > 2 {
            Diagnostic::spanned(tokens[2].span(), Level::Error, UNEXPECTED_TOKEN.to_string())
                .help(ATTRIBUTES_SYNTAX.to_string())
                .abort();
        }

        let kind = tokens.pop_front().unwrap(); // `tokens` contains at least a `kind`
        let body = tokens.pop_front(); // `tokens` might not contain any `body`

        // The first element of `attribute_tokens` must be an `Ident`

        let TokenTree::Ident(kind) = kind else {
            Diagnostic::spanned(
                kind.span(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string()
            ).help(ATTRIBUTES_SYNTAX.to_string())
            .abort();
        };

        // The second element of `attribute_tokens`, if it exists, must be a
        // parenthesized block with the body

        let body = body.map(|body| {
            let TokenTree::Group(body) = body else {
                Diagnostic::spanned(
                    body.span(),
                    Level::Error,
                    UNEXPECTED_TOKEN.to_string()
                ).help(ATTRIBUTES_SYNTAX.to_string())
                .abort();
            };

            if body.delimiter() != Delimiter::Parenthesis {
                Diagnostic::spanned(body.span_open(), Level::Error, UNEXPECTED_TOKEN.to_string())
                    .help(ATTRIBUTES_SYNTAX.to_string())
                    .abort();
            }

            body
        });

        // All checks successful: return `kind` and `body`

        Some((kind, body))
    }
}
