use std::fmt::{self, Display, Formatter};

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
