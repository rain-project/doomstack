use crate::doom::{Description, KeepOriginal, Wrap};

/// A setting pertaining to the derivation of a group of fields (struct or enum variant).
///
/// A [`Setting`] is one of the fields in an [`Attribute`] (which also contains derive-related
/// information such as [`Spans`]).
///
/// [`Attribute`]: crate::doom::Attribute
/// [`Spans`]: crate::doom::attribute::Spans
pub(crate) enum Setting {
    Description(Description),
    KeepOriginal(KeepOriginal),
    Wrap(Wrap),
}
