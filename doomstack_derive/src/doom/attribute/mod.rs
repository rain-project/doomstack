use crate::doom::{
    messages::{errors::*, helps::*},
    Setting,
};
use proc_macro2::Span;
use proc_macro_error::{Diagnostic, Level};

/// A `Doom`-specific attribute of a group of fields (struct or enum variant).
///
/// An [`Attribute`] captures a [`Setting`] of the `Doom` derive, along with useful [`Spans`] for
/// meaningful error reporting. Unlike a [`syn::Attribute`], which just captures a sequence of
/// tokens, an [`Attribute`] must successfully parse a meaningful, well-formed [`Setting`].
pub(crate) struct Attribute {
    pub spans: Spans,
    pub setting: Setting,
}

/// Useful [`Span`]s for meaningful error reporting.
pub(crate) struct Spans {
    pub kind: Span,
}

impl Attribute {
    /// Parses a [`syn::Attribute`] into an [`Attribute`].
    ///
    /// [`Attribute::parse`] returns an [`Option<Attribute>`] to gracefully discard
    /// [`syn::Attribute`]s that do not pertain to the `Doom` derive. `Doom` attributes are all in
    /// the form `#[doom(...)]`: when provided with a `#[foreign ...]` attribute,
    /// [`Attribute::parse`] simply returns [`None`].
    pub fn parse(attribute: &syn::Attribute) -> Option<Self> {
        // `Doom` attributes are in the form `#[doom(kind)]` or `#[doom(kind(body))]`.
        // If `attribute` is a `Doom` attribute, parse `kind` and (optional) `body`.
        // Otherwise, return `None`.

        let (kind, body) = Attribute::parse_parts(attribute)?;

        let spans = Spans { kind: kind.span() };

        // Depending on `kind`, parse `body` into the relevant `Setting`
        // (abort if `kind` is unknown)

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
