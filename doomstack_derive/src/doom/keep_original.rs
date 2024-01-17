use proc_macro2::TokenTree;

/// A [`Setting`] prescribing a group of fields (struct or enum variant) to `Doom::keep_original()`.
///
/// The `condition` field optionally contains a sequence of tokens representing an expression (which
/// is expected to evaluate to a `bool`) capturing the condition for the group of fields to
/// `Doom::keep_original()`. If no `condition` is specified, [`KeepOriginal`] prescribes the group
/// of fields to _always_ `Doom::keep_original()`. To never `Doom::keep_original()`, the user simply
/// omits [`KeepOriginal`] entirely.
///
/// Note: when applied to an enum variant `E::V`, [`KeepOriginal`] prescribes to return `true` (or
/// `condition`, if `condition` applies) whenever `E`'s variant is `V`. Clearly, it is the enum
/// that gets stored - there is no way to store a variant in isolation.
///
/// [`Setting`]: crate::doom::Setting
pub(crate) struct KeepOriginal {
    pub condition: Option<Vec<TokenTree>>,
}
