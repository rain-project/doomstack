use proc_macro2::Ident;
use syn::Type;

pub(crate) enum Fields {
    Named(Vec<(Type, Ident)>),
    Unnamed(Vec<Type>),
    Unit,
}

impl Fields {
    pub fn parse(fields: &syn::Fields) -> Self {
        match fields {
            syn::Fields::Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|field| (field.ty.clone(), field.ident.clone().unwrap()))
                    .collect();

                Fields::Named(fields)
            }

            syn::Fields::Unnamed(fields) => {
                let types = fields
                    .unnamed
                    .iter()
                    .map(|field| field.ty.clone())
                    .collect();

                Fields::Unnamed(types)
            }

            syn::Fields::Unit => Fields::Unit,
        }
    }
}
