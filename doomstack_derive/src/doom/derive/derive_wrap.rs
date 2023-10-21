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
        let item = if let Some(variant) = variant {
            quote!(#identifier::#variant)
        } else {
            quote!(#identifier)
        };

        match fields {
            Fields::Named(fields) => {
                let field_types = fields.iter().map(|(field_type, _)| field_type);
                let field_identifiers = fields.iter().map(|(_, field_identifier)| field_identifier);

                // Note that `()` is the unit type, and `(T)` is the same as `T`: no
                // extra logic is needed to handle `fields` having 0 or 1 elements.
                let argument_type = quote!((#(#field_types),*));

                let binds = field_identifiers
                    .enumerate()
                    .map(|(index, field_identifier)| {
                        // If `fields` has 0 elements, this never gets executed. If
                        // `fields` has exactly 1 element, then `#argument_type` is
                        // not a tuple (remember: `(T)` is the same as `T`). In that
                        // case, we directly assign `argument` to `#field_identifier`:
                        // noting that `index` is necessarily `0`, `argument.#index`
                        // would `quote!` to `argument.0`, accessing the first item
                        // of a non-tuple type and causing a compilation error. If
                        // `fields` has more than one element, then `#argument_type`
                        // is a tuple. In that case, we assign `argument.#index` (the
                        // `index`-th element of `argument`) to `#field_identifier`.
                        if fields.len() == 1 {
                            quote!(#field_identifier: argument)
                        } else {
                            let index = Index {
                                index: index as u32,
                                span: Span::call_site(),
                            };

                            quote!(#field_identifier: argument.#index)
                        }
                    });

                quote! {
                    impl #identifier {
                        pub fn #constructor(argument: #argument_type) -> Self {
                            #item {
                                #(#binds),*
                            }
                        }
                    }
                }
            }

            Fields::Unnamed(types) => {
                // Note that `()` is the unit type, and `(T)` is the same as `T`: no
                // extra logic is needed to handle `types` having 0 or 1 elements.
                let argument_type = quote!((#(#types),*));

                let binds = (0..types.len()).map(|index| {
                    // If `types` has 0 elements, this never gets executed. If `types`
                    // has exactly 1 element, then `#argument_type` is not a tuple
                    // (remember: `(T)` is the same as `T`). In that case, we build
                    // `#item` with `argument` as (only) unnamed argument: noting
                    // that `index` is necessarily `0`, `argument.#index` would
                    // `quote!` to `argument.0`, accessing  the first item of a
                    // non-tuple type and causing a compilation  error. If `fields`
                    // has more than one element, then `argument`is a tuple, and we
                    // can specify `argument.#index` (the `index`-th element of
                    // `argument`) as the `index`-th element of tuple-like `item`.
                    if types.len() == 1 {
                        quote!(argument)
                    } else {
                        let index = Index {
                            index: index as u32,
                            span: Span::call_site(),
                        };

                        quote!(argument.#index)
                    }
                });

                quote! {
                    impl #identifier {
                        pub fn #constructor(argument: #argument_type) -> Self {
                            #item(#(#binds),*)
                        }
                    }
                }
            }

            Fields::Unit => {
                quote! {
                    impl #identifier {
                        pub fn #constructor<A>(argument: A) -> Self {
                            #item
                        }
                    }
                }
            }
        }
    }
}
