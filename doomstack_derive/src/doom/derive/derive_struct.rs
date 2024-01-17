use crate::doom::{Derive, Description, Fields, Settings};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Index;

impl Derive {
    /// Derives the `Doom` trait for a struct.
    pub(in crate::doom::derive) fn derive_struct_doom(
        identifier: &Ident,
        settings: &Settings,
        fields: &Fields,
    ) -> TokenStream {
        // Derive `Doom::tag`

        let tag = quote! {
            fn tag(&self) -> &'static str {
                stringify!(#identifier)
            }
        };

        // Derive the struct's field binds, enabling the user to reference fields directly from
        // `#[doom(description(...))]` or `#[doom(keep_original(...))]`:
        //  - If the struct is named, a reference to each field is bound to a variable of the same
        //    name.
        //  - If the struct is unnamed, a reference to the `n`-th member is bound to a variable
        //    named `_#n`.
        //  - If the struct is a unit, no binds are necessary.

        let binds = match fields {
            Fields::Named(fields) => fields
                .iter()
                .map(|(_, identifier)| quote!(let #identifier = &self.#identifier;))
                .collect(),

            Fields::Unnamed(types) => (0..types.len() as u32)
                .map(|index| {
                    let identifier = Ident::new(format!("_{index}",).as_str(), Span::call_site());

                    let index = Index {
                        index,
                        span: Span::call_site(),
                    };

                    quote!(let #identifier = &self.#index;)
                })
                .collect(),

            Fields::Unit => Vec::new(),
        };

        // Derive description format: just the format literal if static,
        // dynamically formatted with `format!` if owned

        let format = match &settings.description {
            Description::Static { description } => {
                quote!(doomstack::Description::Static(#description))
            }
            Description::Owned { format, arguments } => quote!(doomstack::Description::Owned(
                format!(#format #(#arguments)*)
            )),
        };

        // Derive `Doom::description`

        let description = quote! {
            fn description(&self) -> doomstack::Description {
                #(#binds)*
                #format
            }
        };

        // Derive (optional) `Doom::keep_original` if prescribed by `settings`

        // Note: a default implementation of `Doom::keep_original` is provided, which
        // always returns `false`.

        let keep_original = settings.keep_original.as_ref().map(|keep_original| {
            let condition = keep_original
                .condition
                .as_ref()
                .map(|condition| quote!(#(#condition)*))
                .unwrap_or(quote!(true));

            quote! {
                fn keep_original(&self) -> bool {
                    #(#binds)*
                    #condition
                }
            }
        });

        // Derive `Doom` trait

        quote! {
            impl doomstack::Doom for #identifier {
                #tag
                #description
                #keep_original
            }
        }
    }

    pub(in crate::doom::derive) fn derive_struct_wrap(
        identifier: &Ident,
        settings: &Settings,
        fields: &Fields,
    ) -> Option<TokenStream> {
        settings
            .wrap
            .as_ref()
            .map(|wrap| Derive::derive_wrap(identifier, None, &wrap.constructor, fields))
    }
}
