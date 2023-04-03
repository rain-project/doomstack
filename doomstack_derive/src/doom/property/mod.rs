use syn::{Attribute, Ident, LitStr};

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

impl Property {
    pub fn parse(attribute: &Attribute) -> Option<Self> {
        let (_kind, _body) = Property::tokens(attribute)?;
        todo!()
    }
}

mod messages;
mod tokens;
