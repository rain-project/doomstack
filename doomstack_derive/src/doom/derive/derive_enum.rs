use crate::doom::{derive::Variant, Derive, Description, Fields};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;

impl Derive {
    pub(in crate::doom::derive) fn derive_enum_doom(
        identifier: &Ident,
        variants: &[Variant],
    ) -> TokenStream {
        let tag_branches = variants
            .iter()
            .map(|variant| {
                let variant_identifier = &variant.identifier;

                let variant_tag = format!("{}::{}", identifier, variant_identifier);
                let variant_tag = LitStr::new(variant_tag.as_str(), Span::call_site());

                quote!(#identifier::#variant_identifier { .. } => #variant_tag)
            })
            .collect::<Vec<_>>();

        let tag = quote! {
            fn tag(&self) -> &'static str {
                match self {
                    #(#tag_branches),*
                }
            }
        };

        let description_branches = variants.iter().map(|variant| {
            let variant_identifier = &variant.identifier;

            let bind = match &variant.fields {
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
            };

            let format = match &variant.settings.description {
                Description::Static { description } => {
                    quote!(doomstack::Description::Static(#description))
                }
                Description::Owned { format, arguments } => quote!(doomstack::Description::Owned(
                    format!(#format #(#arguments)*)
                )),
            };

            quote! {
                #identifier::#variant_identifier #bind => {
                    #format
                }
            }
        });

        let description = quote! {
            fn description(&self) -> doomstack::Description {
                match self {
                    #(#description_branches)*
                }
            }
        };

        quote! {
            impl doomstack::Doom for #identifier {
                #tag
                #description
            }
        }
    }

    pub(in crate::doom::derive) fn derive_enum_wraps(
        identifier: &Ident,
        variants: &[Variant],
    ) -> Vec<TokenStream> {
        variants
            .iter()
            .filter_map(|variant| {
                let Some(wrap) = &variant.settings.wrap else {
                    return None;
                };

                Some(Derive::derive_wrap(
                    identifier,
                    Some(&variant.identifier),
                    &wrap.constructor,
                    &variant.fields,
                ))
            })
            .collect()
    }
}
