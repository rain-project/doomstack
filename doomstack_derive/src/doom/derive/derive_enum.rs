use crate::doom::{derive::Variant, Derive};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::LitStr;

impl Derive {
    pub(in crate::doom::derive) fn derive_enum(
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

        let description = quote! {
            fn description(&self) -> doomstack::Description {
                todo!()
            }
        };

        quote! {
            impl doomstack::Doom for #identifier {
                #tag
                #description
            }
        }
    }
}
