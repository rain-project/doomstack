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
    pub fn parse(_attribute: Attribute) -> Option<Self> {
        todo!()
    }
}
