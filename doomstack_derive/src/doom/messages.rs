// TODO: Organize messages by type / improve sorting
// TODO: Refactor `STYLE` into `SYNTAX in const names`
#[rustfmt::skip]
pub(in crate::doom) mod errors {
    pub const EMPTY_ATTRIBUTE: &str = "empty `doom()` attribute";
    pub const UNEXPECTED_TOKEN: &str = "unexpected token in `doom()` attribute";
    pub const UNEXPECTED_KIND: &str = "unexpected `doom()` attribute";
    pub const MISSING_WRAP_BODY: &str = "missing body in `wrap()` attribute";
    pub const MISSING_WRAPPING_CONSTRUCTOR: &str = "missing constructor in `wrap()` attribute";
    pub const UNEXPECTED_WRAP_TOKEN: &str = "unexpected token in `wrap()` attribute";
    pub const MISSING_DESCRIPTION_BODY: &str = "missing body in `description()` attribute";
    pub const MISSING_DESCRIPTION_FORMAT: &str = "missing format string in `description()` attribute";
    pub const UNEXPECTED_DESCRIPTION_TOKEN: &str = "unexpected token in `description()` attribute";
    pub const UNEXPECTED_DESCRIPTION_ARGUMENTS: &str = "unexpected arguments in `description(format, ...)` attribute: `format` does not format any variable";
    pub const MULTIPLE_DESCRIPTIONS: &str = "multiple `description()` attributes for the same item";
    pub const MULTIPLE_WRAPS: &str = "multiple `wrap()` attributes for the same item";
    pub const MISSING_DESCRIPTION: &str = "missing `description()` attribute";
    pub const UNION_UNDERIVABLE: &str = "deriving `Doom` for a `union` type";
}

#[rustfmt::skip]
pub(in crate::doom) mod helps {
    pub const ATTRIBUTES_SYNTAX: &str =
          r#"`doom()` attributes look like tags or function calls:
          `#[doom(attribute)]` or `#[doom(attribute(...))]`"#;

    pub const AVAILABLE_KINDS: &str = 
          r#"available `doom()` attributes are: `description`, `wrap`"#;

    pub const WRAP_STYLE: &str = 
          r#"`wrap` attributes take the identifier of the wrapping constructor:
          `#[doom(wrap(my_error))]`"#;

    pub const DESCRIPTION_STYLE: &str = 
          r#"`description` attributes take a format string, possibly formatting error fields:
          `#[doom(description("Error with severity {severity}"))]`
          struct MyError {
              severity: u32,
          }"#;

    pub const SINGLE_DESCRIPTION: &str = 
          r#"each item (`struct` or `enum` variant) must have exactly one `description()` attribute"#;

    pub const OPTIONAL_WRAP: &str = 
          r#"each item (`struct` or `enum` variant) must have at most one `wrap()` attribute"#;

    pub const DERIVABLES: &str = 
          r#"`Doom` can only be derived for `struct` or `enum` types"#;
}
