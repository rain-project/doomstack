#[rustfmt::skip]
pub(in crate::doom::property) mod errors {
    pub const INCOMPLETE_ATTRIBUTE: &str = "incomplete `doom()` attribute";
    pub const UNEXPECTED_TOKEN: &str = "unexpected token in `doom()` attribute";
    pub const UNEXPECTED_KIND: &str = "unexpected `doom()` attribute";
    pub const MISSING_WRAPPING_CONSTRUCTOR: &str = "missing constructor in `wrap()` attribute";
    pub const UNEXPECTED_WRAP_TOKEN: &str = "unexpected token in `wrap()` attribute";
}

#[rustfmt::skip]
pub(in crate::doom::property) mod helps {
    pub const ATTRIBUTES_LIKE_FUNCTIONS: &str =
          r#"`doom()` attributes look like function calls:
          `#[doom(attribute(...))]`"#;

    pub const AVAILABLE_KINDS: &str = 
          r#"available `doom()` attributes are: `description`, `wrap`"#;

    pub const WRAP_STYLE: &str = 
          r#"`wrap` attributes take the identifier of the wrapping constructor:
          `#[doom(wrap(my_error))]`"#;
}
