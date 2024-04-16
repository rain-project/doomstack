use crate::doom::{derive::Variant, Derive, Description, Fields};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;

impl Derive {
    /// Derives the `Doom` trait for an enum.
    pub(in crate::doom::derive) fn derive_enum_doom(
        identifier: &Ident,
        variants: &[Variant],
    ) -> TokenStream {
        // Derive variant branches for `Doom::tag`

        let tag_branches = variants
            .iter()
            .map(|variant| {
                let variant_identifier = &variant.identifier;

                // Each variant `#identifier::#variant_identifier` is mapped
                // onto a literal "#identifier::#variant_identifier".

                let variant_tag = format!("{}::{}", identifier, variant_identifier);
                let variant_tag = LitStr::new(variant_tag.as_str(), Span::call_site());

                quote!(#identifier::#variant_identifier { .. } => #variant_tag)
            })
            .collect::<Vec<_>>();

        // Derive `Doom::tag`

        let tag = quote! {
            fn tag(&self) -> &'static str {
                match self {
                    #(#tag_branches),*
                }
            }
        };

        // Derive variant branches for `Doom::description`

        let description_branches = variants.iter().map(|variant| {
            let variant_identifier = &variant.identifier;
            let variant_binds = Derive::derive_variant_binds(&variant.fields);

            // Derive description format: just the format literal if static,
            // dynamically formatted with `format!` if owned

            let format = match &variant.settings.description {
                Description::Static { description } => {
                    quote!(doomstack::Description::Static(#description))
                }
                Description::Owned { format, arguments } => quote!(doomstack::Description::Owned(
                    format!(#format #(#arguments)*)
                )),
            };

            // Derive match arm for description variant

            quote! {
                #identifier::#variant_identifier #variant_binds => {
                    #format
                }
            }
        });

        // Derive `Doom::description`

        let description = quote! {
            fn description(&self) -> doomstack::Description {
                match self {
                    #(#description_branches)*
                }
            }
        };

        // Derive variant branches for `Doom::keep_original`

        let keep_original_branches = variants.iter().map(|variant| {
            let variant_identifier = &variant.identifier;
            let variant_binds = Derive::derive_variant_binds(&variant.fields);

            // Derive `Doom::keep_original` arm as prescribed by `settings`

            let condition = if let Some(keep_original) = &variant.settings.keep_original {
                if let Some(condition) = &keep_original.condition {
                    quote!(#(#condition)*)
                } else {
                    quote!(true)
                }
            } else {
                // Note: unlike in the struct case, here we cannot rely on the default
                // implementation of `Doom::keep_original` returning `false`: we have already
                // started overriding the default implementation by matching the enum! Every variant
                // for which `#[doom(keep_original)]` / `#[doom(keep_original(...))]` is not
                // specified must explicitly return `false`. A special case could be implemented
                // to skip deriving `Doom::keep_original` entirely if no variant is tagged with
                // `#[doom(keep_original)]` / `#[doom(keep_original(...))]`, but doing so would
                // unnecessarily add to the code complexity. A match where all arms point to `false`
                // is optimized away by the compiler.

                quote!(false)
            };

            quote! {
                #identifier::#variant_identifier #variant_binds => {
                    #condition
                }
            }
        });

        // Derive `Doom::keep_original`

        let keep_original = quote! {
            fn keep_original(&self) -> bool {
                match self {
                    #(#keep_original_branches)*
                }
            }
        };

        // Derive `Doom` trait

        quote! {
            impl doomstack::Doom for #identifier {
                #tag
                #description
                #keep_original
            }
        }
    }

    /// Derives all wrapping constructors for an enum.
    pub(in crate::doom::derive) fn derive_enum_wraps(
        identifier: &Ident,
        variants: &[Variant],
    ) -> Vec<TokenStream> {
        variants
            .iter()
            .filter_map(|variant| {
                let wrap = variant.settings.wrap.as_ref()?;

                Some(Derive::derive_wrap(
                    identifier,
                    Some(&variant.identifier),
                    &wrap.constructor,
                    &variant.fields,
                ))
            })
            .collect()
    }

    /// Derives the field binds for an enum variant in a match arm.
    ///
    ///  - Mentions all fields by name if the variant is named;
    ///  - Enumerates all fields by index, prefixed by an underscore (`_0`, `_1`, ...), if the
    ///    variant is unnamed;
    ///  - Returns nothing if the variant is a unit.
    fn derive_variant_binds(fields: &Fields) -> TokenStream {
        match fields {
            Fields::Named(fields) => {
                let fields = fields.iter().map(|(_, identifier)| identifier);
                quote!({#(#fields),*})
            }

            Fields::Unnamed(fields) => {
                let fields = (0..fields.len())
                    .map(|index| Ident::new(format!("_{index}",).as_str(), Span::call_site()));

                quote!((#(#fields),*))
            }

            Fields::Unit => quote!(),
        }
    }
}
