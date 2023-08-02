use crate::doom::{
    messages::{errors::*, helps::*},
    Attribute, Description, Fields, Settings,
};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::{Diagnostic, Level};
use quote::quote;
use syn::{Data, DeriveInput, Index};

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

pub(crate) struct Variant {
    identifier: Ident,
    settings: Settings,
    fields: Fields,
}

impl Derive {
    pub fn parse(input: &DeriveInput) -> Self {
        let identifier = input.ident.clone();

        match &input.data {
            Data::Struct(data) => {
                let attributes = input.attrs.iter().filter_map(Attribute::parse);
                let settings = Settings::from_attributes(attributes, identifier.span());
                let fields = Fields::parse(&data.fields);

                Derive::Struct {
                    identifier,
                    settings,
                    fields,
                }
            }

            Data::Enum(data) => {
                let variants = data
                    .variants
                    .iter()
                    .map(|variant| {
                        let identifier = variant.ident.clone();
                        let attributes = variant.attrs.iter().filter_map(Attribute::parse);
                        let settings = Settings::from_attributes(attributes, identifier.span());
                        let fields = Fields::parse(&variant.fields);

                        Variant {
                            identifier,
                            settings,
                            fields,
                        }
                    })
                    .collect();

                Derive::Enum {
                    identifier,
                    variants,
                }
            }

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

    pub fn derive(&self) -> TokenStream {
        match self {
            Derive::Struct {
                identifier,
                settings,
                fields,
            } => {
                let tag = quote! {
                    fn tag(&self) -> &'static str {
                        stringify!(#identifier)
                    }
                };

                let binds = match fields {
                    Fields::Named(fields) => fields
                        .iter()
                        .map(|(_, identifier)| quote!(let #identifier = self.#identifier;))
                        .collect(),

                    Fields::Unnamed(types) => (0..types.len() as u32)
                        .into_iter()
                        .map(|index| {
                            (
                                Ident::new(format!("_{index}",).as_str(), Span::call_site()),
                                index,
                            )
                        })
                        .map(|(bind, index)| {
                            let index = Index {
                                index,
                                span: Span::call_site(),
                            };

                            quote!(let #bind = self.#index;)
                        })
                        .collect(),

                    Fields::Unit => Vec::new(),
                };

                let format = match &settings.description {
                    Description::Static { description } => {
                        quote!(doomstack::Description::Static(#description))
                    }
                    Description::Owned { format, arguments } => quote!(
                        doomstack::Description::Owned(format!(#format, #(#arguments),*))
                    ),
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

            Derive::Enum { .. } => {
                todo!()
            }
        }
    }
}
