use crate::doom::{Derive, Fields};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Index;

impl Derive {
    /// Derives a wrapping constructor for a group of fields (struct or enum variant).
    ///
    /// Expects:
    ///  - The `identifier` of the type (struct or enum);
    ///  - The (optional) identifier of the `variant` (to be provided if the type is an enum);
    ///  - The wrapping `constructor`'s name;
    ///  - The struct or variant's `fields`.
    pub(in crate::doom::derive) fn derive_wrap(
        identifier: &Ident,
        variant: Option<&Ident>,
        constructor: &Ident,
        fields: &Fields,
    ) -> TokenStream {
        // Derive the item's path (`identifier` if the item is a struct,
        // `identifier::variant` if the item is an enum)

        let item = if let Some(variant) = variant {
            quote!(#identifier::#variant)
        } else {
            quote!(#identifier)
        };

        // Derive the wrapping constructor

        match fields {
            Fields::Named(fields) => {
                let field_types = fields.iter().map(|(field_type, _)| field_type);
                let field_identifiers = fields.iter().map(|(_, field_identifier)| field_identifier);

                // Derive the type of the wrapping constructor's argument

                // Note that `()` is the unit type, and `(T)` is the same as `T`: no
                // extra logic is needed to handle `fields` having 0 or 1 elements.
                let argument_type = quote!((#(#field_types),*));

                // Derive the item's field binds (each of the argument's members
                // must be assigned to each of the item's fields)

                let binds = field_identifiers
                    .enumerate()
                    .map(|(index, field_identifier)| {
                        // If `fields` has 0 elements, this never gets executed. If `fields` has
                        // exactly 1 element, then `#argument_type` is not a tuple (remember: `(T)`
                        // is the same as `T`). In that case, we directly assign `argument` to
                        // `#field_identifier`: noting that `index` is necessarily `0`,
                        // `argument.#index` would `quote!` to `argument.0`, accessing the first
                        // item of a non-tuple type and causing a compilation error. If `fields` has
                        // more than one element, then `#argument_type` is a tuple. In that case, we
                        // assign `argument.#index` (the`index`-th element of `argument`) to
                        // `#field_identifier`.
                        if fields.len() == 1 {
                            quote!(#field_identifier: argument.into())
                        } else {
                            let index = Index {
                                index: index as u32,
                                span: Span::call_site(),
                            };

                            quote!(#field_identifier: argument.#index.into())
                        }
                    });

                // Derive the wrapping constructor

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
                // Derive the type of the wrapping constructor's argument

                // Note that `()` is the unit type, and `(T)` is the same as `T`: no
                // extra logic is needed to handle `types` having 0 or 1 elements.
                let argument_type = quote!((#(#types),*));

                // Derive the item's field binds (each of the argument's members
                // must be listed in sequence to construct the tuple-like item)

                let binds = (0..types.len()).map(|index| {
                    // If `types` has 0 elements, this never gets executed. If `types` has exactly 1
                    // element, then `#argument_type` is not a tuple (remember: `(T)` is the same as
                    // `T`). In that case, we build `#item` with `argument` as (only) unnamed
                    // argument: noting that `index` is necessarily `0`, `argument.#index` would
                    // `quote!` to `argument.0`, accessing  the first item of a non-tuple type and
                    // causing a compilation  error. If `fields` has more than one element, then
                    // `argument`is a tuple, and we can specify `argument.#index` (the `index`-th
                    // element of `argument`) as the `index`-th element of tuple-like `item`.
                    if types.len() == 1 {
                        quote!(argument.into())
                    } else {
                        let index = Index {
                            index: index as u32,
                            span: Span::call_site(),
                        };

                        quote!(argument.#index.into())
                    }
                });

                // Derive the wrapping constructor

                quote! {
                    impl #identifier {
                        pub fn #constructor(argument: #argument_type) -> Self {
                            #item(#(#binds),*)
                        }
                    }
                }
            }

            Fields::Unit => {
                // The wrapping constructor takes one argument of any time,
                // ignores it, and returns the unit item.

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
