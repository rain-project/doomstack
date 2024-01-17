use proc_macro2::Ident;
use syn::Type;

/// The fields in a group of fields (struct or enum variant).
///
/// Can be: [`Named`] (storing each field's type and identifier); [`Unnamed`] (or tuple-like,
/// storing each field's type); or [`Unit`] (no field at all).
///
/// Note: a [`Named`] / [`Unnamed`] group of fields that contains no fields is still [`Named`] /
/// [`Unnamed`], not [`Unit`]. It is the type of bracket (`{}` for [`Named`], `()` for [`Unnamed`],
/// absent for [`Unit`]) that determines the variant of [`Fields`].
///
/// [`Named`]: Fields::Named
/// [`Unit`]: Fields::Unit
/// [`Unnamed`]: Fields::Unnamed
pub(crate) enum Fields {
    Named(Vec<(Type, Ident)>),
    Unnamed(Vec<Type>),
    Unit,
}

impl Fields {
    /// Parses a [`Fields`] from a [`syn::Fields`].
    pub fn parse(fields: &syn::Fields) -> Self {
        match fields {
            syn::Fields::Named(fields) => {
                // Extract the type and identifier of each field

                let fields = fields
                    .named
                    .iter()
                    .map(|field| (field.ty.clone(), field.ident.clone().unwrap()))
                    .collect();

                Fields::Named(fields)
            }

            syn::Fields::Unnamed(fields) => {
                // Extract the type of each field

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
