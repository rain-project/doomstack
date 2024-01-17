use proc_macro2::TokenTree;
use syn::LitStr;

/// A [`Setting`] prescribing the `Doom::description()` of a group of fields (struct or enum
/// variant).
///
/// As we discuss in the main doomstack crate, a [`Description`] can be either [`Static`] (in which
/// case a [`LitStr`] `description` is sufficient) or [`Owned`] (in which case the `arguments` field
/// captures all the additional arguments to format, following the `format` [`LitStr`]).
///
/// Note: in the [`Owned`] case, `arguments` captures all tokens following `format`: this includes
/// whatever comma separates the format string from the arguments, or the arguments from each other.
/// When deriving, do not interleave commas between the items of `arguments`!
///
/// [`LitStr`]: struct@LitStr
/// [`Owned`]: Description::Owned
/// [`Setting`]: crate::doom::Setting
/// [`Static`]: Description::Static
pub(crate) enum Description {
    Static {
        description: LitStr,
    },
    Owned {
        format: LitStr,
        arguments: Vec<TokenTree>,
    },
}
