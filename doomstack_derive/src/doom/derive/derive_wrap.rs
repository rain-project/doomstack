use crate::doom::{Derive, Fields};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Index;

impl Derive {
    pub(in crate::doom::derive) fn derive_wrap(
        identifier: &Ident,
        variant: Option<&Ident>,
        constructor: &Ident,
        fields: &Fields,
    ) -> TokenStream {
        let name = if let Some(variant) = variant {
            quote!(#identifier::#variant)
        } else {
            quote!(#identifier)
        };

        match fields {
            Fields::Named(fields) => {
                let field_types = fields.iter().map(|(field_type, _)| field_type);
                let field_identifiers = fields.iter().map(|(_, field_identifier)| field_identifier);

                let argument_type = quote!((#(#field_types),*));

                let binds = field_identifiers
                    .enumerate()
                    .map(|(index, field_identifier)| {
                        let index = Index {
                            index: index as u32,
                            span: Span::call_site(),
                        };

                        quote!(#field_identifier: argument.#index)
                    });

                quote! {
                    impl #identifier {
                        pub fn #constructor(argument: #argument_type) -> Self {
                            #name {
                                #(#binds),*
                            }
                        }
                    }
                }
            }

            Fields::Unnamed(types) => {
                let argument_type = quote!((#(#types),*));

                let binds = (0..types.len()).map(|index| {
                    let index = Index {
                        index: index as u32,
                        span: Span::call_site(),
                    };

                    quote!(argument.#index)
                });

                quote! {
                    impl #identifier {
                        pub fn #constructor(argument: #argument_type) -> Self {
                            #name(#(#binds),*)
                        }
                    }
                }
            }

            Fields::Unit => {
                quote! {
                    impl #identifier {
                        pub fn #constructor<A>(argument: A) -> Self {
                            #name
                        }
                    }
                }
            }
        }
    }
}
