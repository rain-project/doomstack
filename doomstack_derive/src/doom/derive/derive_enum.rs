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
            let variant_binds = Derive::derive_variant_binds(&variant.fields);

            let format = match &variant.settings.description {
                Description::Static { description } => {
                    quote!(doomstack::Description::Static(#description))
                }
                Description::Owned { format, arguments } => quote!(doomstack::Description::Owned(
                    format!(#format #(#arguments)*)
                )),
            };

            quote! {
                #identifier::#variant_identifier #variant_binds => {
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

        let keep_original_branches = variants.iter().map(|variant| {
            let variant_identifier = &variant.identifier;
            let variant_binds = Derive::derive_variant_binds(&variant.fields);

            let condition = if let Some(keep_original) = &variant.settings.keep_original {
                if let Some(condition) = &keep_original.condition {
                    quote!(#(#condition)*)
                } else {
                    quote!(true)
                }
            } else {
                quote!(false)
            };

            quote! {
                #identifier::#variant_identifier #variant_binds => {
                    #condition
                }
            }
        });

        let keep_original = quote! {
            fn keep_original(&self) -> bool {
                match self {
                    #(#keep_original_branches)*
                }
            }
        };

        quote! {
            impl doomstack::Doom for #identifier {
                #tag
                #description
                #keep_original
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
