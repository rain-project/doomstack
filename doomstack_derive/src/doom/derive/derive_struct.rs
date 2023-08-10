use crate::doom::{Derive, Description, Fields, Settings};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Index;

impl Derive {
    pub(in crate::doom::derive) fn derive_struct(
        identifier: &Ident,
        settings: &Settings,
        fields: &Fields,
    ) -> TokenStream {
        let tag = quote! {
            fn tag(&self) -> &'static str {
                stringify!(#identifier)
            }
        };

        let binds = match fields {
            Fields::Named(fields) => fields
                .iter()
                .map(|(_, identifier)| quote!(let #identifier = &self.#identifier;))
                .collect(),

            Fields::Unnamed(types) => (0..types.len() as u32)
                .into_iter()
                .map(|index| {
                    let bind = Ident::new(format!("_{index}",).as_str(), Span::call_site());
                    let index = Index {
                        index,
                        span: Span::call_site(),
                    };

                    quote!(let #bind = &self.#index;)
                })
                .collect(),

            Fields::Unit => Vec::new(),
        };

        let format = match &settings.description {
            Description::Static { description } => {
                quote!(doomstack::Description::Static(#description))
            }
            Description::Owned { format, arguments } => quote!(doomstack::Description::Owned(
                format!(#format, #(#arguments),*)
            )),
        };

        let description = quote! {
            fn description(&self) -> doomstack::Description {
                #(#binds)*
                #format
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
