use crate::{Doom, Entry, Top};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Default, Clone)]
pub struct Stack {
    entries: Vec<Entry>,
}

impl Stack {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn entries(&self) -> &[Entry] {
        self.entries.as_slice()
    }

    pub fn push<D>(self, doom: D) -> Top<D>
    where
        D: Doom,
    {
        Top::from_parts(doom, self)
    }

    pub fn push_as_stack<D>(mut self, doom: D) -> Self
    where
        D: Doom,
    {
        self.entries.push(Entry::archive(doom));
        self
    }

    pub fn spot(mut self, location: (&'static str, u32)) -> Self {
        self.entries.last_mut().unwrap().spot(location);
        self
    }

    pub fn pot<P>(self, doom: P, location: (&'static str, u32)) -> Top<P>
    where
        P: Doom,
    {
        self.push(doom).spot(location)
    }
}

impl Error for Stack {}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "<top: {}>", self.entries.last().unwrap())
    }
}

impl Debug for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for entry in self.entries.iter().rev() {
            writeln!(f, "{entry:?}")?;
        }

        Ok(())
    }
}
