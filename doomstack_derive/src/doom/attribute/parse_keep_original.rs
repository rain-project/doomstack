use crate::doom::{Attribute, KeepOriginal};
use proc_macro2::Group;

impl Attribute {
    /// Parses the body of a `keep_original` attribute into a [`KeepOriginal`]
    ///
    /// Inputs the (optional) `body` of a `#[doom(keep_original)]` or `#[doom(keep_original(body))]` attribute.
    /// Expects `body` (if it exists) to be a boolean expression representing the condition to `keep_original`.
    /// Returns a [`KeepOriginal`].
    pub(in crate::doom::attribute) fn parse_keep_original(body: Option<Group>) -> KeepOriginal {
        let condition = body.map(|body| body.stream().into_iter().collect::<Vec<_>>());
        KeepOriginal { condition }
    }
}
