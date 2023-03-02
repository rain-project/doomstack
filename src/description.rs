use std::fmt::{self, Debug, Display, Formatter};

/// An error description. To maximize efficiency, a `Description` can either be
/// a `Static` (`&'static str`) or `Owned` (`String`) string. This allows
/// `Description`s that are known at compile time to cause no allocations and
/// have no memory footprint at runtime.
#[derive(Clone)]
pub enum Description {
    Static(&'static str),
    Owned(String),
}

impl Description {
    /// Extracts a string slice containing the entire `Description`.
    ///
    /// # Examples
    ///
    /// ```
    /// use doomstack::Description;
    ///
    /// let description = Description::Owned("Oupsie!".to_owned());
    ///
    /// assert_eq!("Oupsie!", description.as_str());
    /// ```
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
