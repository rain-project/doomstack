use crate::doom::{
    messages::{errors::*, helps::*},
    Attribute,
};
use proc_macro2::{Delimiter, Group, TokenTree};
use proc_macro_error::{Diagnostic, Level};
use std::collections::VecDeque;
use syn::{Ident, Meta};

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
        // `attribute` is in one of three forms: `#[scope]`, `#[scope(tokens)]` or
        // `#[scope = value]`, where `scope` is a `::`-separated sequence of segments.

        // Extract `scope` and optional `tokens` (`tokens` is `Some` only
        // if `attribute` is in the form `#[scope(tokens)]`, see above)

        let (scope, tokens) = match &attribute.meta {
            Meta::List(meta) => (
                &meta.path,
                Some(meta.tokens.clone().into_iter().collect::<VecDeque<_>>()),
            ),

            Meta::Path(scope) => (scope, None),
            Meta::NameValue(meta) => (&meta.path, None),
        };

        // Return `None` if `scope` is not "doom" (in that case, `attribute` does
        // not pertain to the `Doom` derive, and as such it should be ignored)

        let scope = if scope.segments.len() == 1 {
            &scope.segments[0]
        } else {
            // `scope` is a multi-segment path (e.g., `a::b::c`)
            return None;
        };

        if scope.ident != "doom" {
            // `scope` contains a single segment, but that segment is not "doom"
            return None;
        }

        // `scope` is "doom", which means that `attribute` pertains to the `Doom` derive.
        // Every error from this point on should result in an abort, as a failure to parse
        // would mean that `attribute` is malformed.

        // Abort if `attribute` is not in the form `#[doom(tokens)]`

        let Some(mut tokens) = tokens else {
            Diagnostic::spanned(
                scope.ident.span(),
                Level::Error,
                MALFORMED_ATTRIBUTE.to_string(),
            )
            .help(ATTRIBUTES_SYNTAX.to_string())
            .abort();
        };

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
            Diagnostic::spanned(kind.span(), Level::Error, UNEXPECTED_TOKEN.to_string())
                .help(ATTRIBUTES_SYNTAX.to_string())
                .abort();
        };

        // The second element of `attribute_tokens`, if it exists, must be a
        // parenthesized block with the body

        let body = body.map(|body| {
            let TokenTree::Group(body) = body else {
                Diagnostic::spanned(body.span(), Level::Error, UNEXPECTED_TOKEN.to_string())
                    .help(ATTRIBUTES_SYNTAX.to_string())
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
