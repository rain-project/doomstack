use crate::doom::{
    messages::{errors::*, helps::*},
    Fields, Settings,
};
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{Diagnostic, Level};
use quote::quote;
use syn::{Data, DeriveInput};

/// A representation of a struct or enum on which to derive `Doom`, complete with all [`Settings`]
/// parsed from each item's attributes.
///
/// The canonical way to use [`Derive`] is to transform a [`DeriveInput`] using
/// [`Derive::parse`] first, then [`Derive::derive`].
pub(crate) enum Derive {
    Struct {
        identifier: Ident,
        settings: Settings,
        fields: Fields,
    },
    Enum {
        identifier: Ident,
        variants: Vec<Variant>,
    },
}

/// A representation of an enum's variant on which to derive `Doom`.
///
/// [`Variant`]s are used in [`Derive::Enum`] to list the variants of an enum.
pub(crate) struct Variant {
    identifier: Ident,
    settings: Settings,
    fields: Fields,
}

impl Derive {
    /// Parses a [`DeriveInput`] into a [`Derive`].
    ///
    /// Expects `input` to be a struct or an enum (`Doom` cannot be derived on unions). Any error
    /// results in a graceful abort, indicating the problem with a meaningful message.
    pub fn parse(input: &DeriveInput) -> Self {
        let identifier = input.ident.clone();
        let attributes = &input.attrs;

        match &input.data {
            Data::Struct(data) => Derive::parse_struct(identifier, attributes, data),
            Data::Enum(data) => Derive::parse_enum(identifier, data),

            Data::Union(data) => {
                Diagnostic::spanned(
                    data.union_token.span,
                    Level::Error,
                    UNION_UNDERIVABLE.to_string(),
                )
                .help(DERIVABLES.to_string())
                .abort();
            }
        }
    }

    /// Derives `Doom` trait and wrapping constructors for the struct or enum that [`Derive::parse`]
    /// parsed to produce `self`.
    pub fn derive(&self) -> TokenStream {
        match self {
            Derive::Struct {
                identifier,
                settings,
                fields,
            } => {
                let doom = Derive::derive_struct_doom(identifier, settings, fields);
                let wraps = Derive::derive_struct_wraps(identifier, settings, fields);

                quote! {
                    #doom
                    #(#wraps)*
                }
            }

            Derive::Enum {
                identifier,
                variants,
            } => {
                let doom = Derive::derive_enum_doom(identifier, variants);
                let wraps = Derive::derive_enum_wraps(identifier, variants);

                quote! {
                    #doom
                    #(#wraps)*
                }
            }
        }
    }
}

mod derive_enum;
mod derive_struct;
mod derive_wrap;
mod parse_enum;
mod parse_struct;
