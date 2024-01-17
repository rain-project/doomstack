use proc_macro2::Ident;

/// A [`Setting`] prescribing the derivation of a wrapping constructor `constructor` for a group of
/// fields (struct or enum variant).
///
/// [`Setting`]: crate::doom::Setting
pub(crate) struct Wrap {
    pub constructor: Ident,
}
