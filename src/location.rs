use std::fmt::{self, Debug, Display, Formatter};

/// The location of a line of code.
///
/// See [`here!()`] for a simple way to get the [`Location`] of a piece of code.
///
/// [`here!()`]: crate::here!
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub file: &'static str,
    pub line: u32,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.file, self.line)
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self, f)
    }
}
