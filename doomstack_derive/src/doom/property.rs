use proc_macro2::{Delimiter, TokenTree};
use proc_macro_error::{Diagnostic, Level};
use syn::{Attribute, Ident, LitStr, Meta};

#[allow(dead_code)]
pub(crate) enum Property {
    StaticDescription {
        description: LitStr,
    },
    OwnedDescription {
        description: LitStr,
        arguments: Vec<Ident>,
    },
    Wrap {
        constructor: Ident,
    },
}

const INCOMPLETE_ATTRIBUTE: &str = "incomplete `doom()` attribute";
const UNEXPECTED_TOKEN: &str = "unexpected token in `doom()` attribute";
const UNEXPECTED_TYPE: &str = "unexpected `doom()` attribute type";
const MISSING_WRAP: &str = "missing constructor in `wrap()` attribute";
const UNEXPECTED_WRAP_TOKEN: &str = "unexpected token in `wrap()` attribute";

const ATTRIBUTES_LIKE_FUNCTIONS: &str =
    r#"`doom()` attributes look like function calls: \n `#[doom(attribute(...))]`"#;

const WRAP_STYLE: &str = r#"`wrap` attributes take the identifier of the wrapping constructor: \n `#[doom(wrap(my_error))]`"#;

const ATTRIBUTE_TYPES: &str = "available `doom()` attribute types are: `description`, `wrap`";

impl Property {
    pub fn parse(attribute: Attribute) -> Option<Property> {
        // If `attribute` is not in the form `#[doom(...)]`, return `None`:
        // `attribute` is not necessarily malformed, but it does not pertain
        // to the `Doom` derive and as such should be ignored

        let Meta::List(meta_list) = &attribute.meta else {
            // `attribute` is not in the form `#[scope(...)]`
            return None;
        };

        let scope_segment = if meta_list.path.segments.len() == 1 {
            &meta_list.path.segments[0]
        } else {
            // `attribute` is in the form `#[scope(...)]`, but `scope`
            // is a multi-segment path (e.g., `a::b::c`)
            return None;
        };

        if scope_segment.ident != "doom" {
            // `attribute` is in the form `#[scope(...)]`, but `scope` is not `doom`
            return None;
        }

        // `attribute` is in the form `#[doom(...)]` and as such it pertains to the
        // `Doom` derive: every error from now on must result in an `abort()`

        let attribute_tokens = meta_list.tokens.clone().into_iter().collect::<Vec<_>>();

        // Note: `attribute_tokens` stores (as `Vec<TokenTree>`) the content
        // of the `#[doom(...)]` attribute. We expect `attribute_tokens` to
        // be in the form of a function call, such as `description(...)`.

        // Two `attribute_tokens` are required for a function-like attribute:
        // the name of the attribute and a parenthesized block with the body

        if attribute_tokens.len() < 2 {
            Diagnostic::spanned(
                attribute.bracket_token.span.close(),
                Level::Error,
                INCOMPLETE_ATTRIBUTE.to_string(),
            )
            .help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        }

        if attribute_tokens.len() > 2 {
            Diagnostic::spanned(
                attribute_tokens[2].span(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string(),
            )
            .help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        }

        // The first element of `attribute_tokens` must be an `Ident`

        let TokenTree::Ident(attribute_type) = &attribute_tokens[0] else {
            Diagnostic::spanned(
                attribute_tokens[0].span(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string()
            ).help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        };

        // The second element of `attribute_tokens` must be a parenthesized block with the body

        let TokenTree::Group(body_group) = &attribute_tokens[1] else {
            Diagnostic::spanned(
                attribute_tokens[1].span(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string()
            ).help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        };

        if body_group.delimiter() != Delimiter::Parenthesis {
            Diagnostic::spanned(
                body_group.span_open(),
                Level::Error,
                UNEXPECTED_TOKEN.to_string(),
            )
            .help(ATTRIBUTES_LIKE_FUNCTIONS.to_string())
            .abort();
        }

        // Parse the body - different constraints are enforced depending on `attribute_type`

        let body_tokens = body_group.stream().into_iter().collect::<Vec<_>>();

        match attribute_type.to_string().as_str() {
            "description" => {
                todo!()
            }
            "wrap" => {
                // The body must contain exactly one `Ident`

                if body_tokens.is_empty() {
                    Diagnostic::spanned(body_group.span(), Level::Error, MISSING_WRAP.to_string())
                        .help(WRAP_STYLE.to_string())
                        .abort();
                }

                if body_tokens.len() > 1 {
                    Diagnostic::spanned(
                        body_tokens[1].span(),
                        Level::Error,
                        UNEXPECTED_WRAP_TOKEN.to_string(),
                    )
                    .help(WRAP_STYLE.to_string())
                    .abort();
                }

                let TokenTree::Ident(constructor) = &body_tokens[0] else {
                    Diagnostic::spanned(
                        body_tokens[0].span(),
                        Level::Error,
                        UNEXPECTED_WRAP_TOKEN.to_string(),
                    )
                    .help(WRAP_STYLE.to_string())
                    .abort();
                };

                Some(Property::Wrap {
                    constructor: constructor.clone(),
                })
            }
            _ => {
                // Unexpected attribute type: must be one of the above

                Diagnostic::spanned(
                    attribute_tokens[0].span(),
                    Level::Error,
                    UNEXPECTED_TYPE.to_string(),
                )
                .help(ATTRIBUTE_TYPES.to_string())
                .abort();
            }
        }
    }
}
