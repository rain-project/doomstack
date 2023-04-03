#[rustfmt::skip]
pub(in crate::doom::property) mod errors {
    pub const INCOMPLETE_ATTRIBUTE: &str = "incomplete `doom()` attribute";
    pub const UNEXPECTED_TOKEN: &str = "unexpected token in `doom()` attribute";
}

#[rustfmt::skip]
pub(in crate::doom::property) mod helps {
    pub const ATTRIBUTES_LIKE_FUNCTIONS: &str =
          r#"`doom()` attributes look like function calls:
          `#[doom(attribute(...))]`"#;
}
