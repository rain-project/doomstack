#[rustfmt::skip]
pub(in crate::doom::property) mod errors {
    pub const INCOMPLETE_ATTRIBUTE: &str = "incomplete `doom()` attribute";
    pub const UNEXPECTED_TOKEN: &str = "unexpected token in `doom()` attribute";
    pub const UNEXPECTED_KIND: &str = "unexpected `doom()` attribute";
}

#[rustfmt::skip]
pub(in crate::doom::property) mod helps {
    pub const ATTRIBUTES_LIKE_FUNCTIONS: &str =
          r#"`doom()` attributes look like function calls:
          `#[doom(attribute(...))]`"#;

    pub const AVAILABLE_KINDS: &str = 
          r#"available `doom()` attributes are: `description`, `wrap`"#;
}
