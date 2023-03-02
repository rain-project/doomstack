use std::fmt::{self, Debug, Display, Formatter};

#[derive(Clone)]
pub enum Description {
    Static(&'static str),
    Owned(String),
}

impl Description {
    pub fn as_str(&self) -> &str {
        match self {
            Description::Static(description) => description,
            Description::Owned(description) => description,
        }
    }
}

impl From<&'static str> for Description {
    fn from(description: &'static str) -> Self {
        Description::Static(description)
    }
}

impl From<String> for Description {
    fn from(description: String) -> Self {
        Description::Owned(description)
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.as_str())
    }
}
