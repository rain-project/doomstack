use crate::{Doom, Entry, Top};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Clone)]
pub struct Stack {
    entries: Vec<Entry>,
}

impl Stack {
    pub(crate) fn new() -> Self {
        Stack {
            entries: Vec::new(),
        }
    }

    pub fn entries(&self) -> &[Entry] {
        self.entries.as_slice()
    }

    pub fn push<D>(mut self, doom: D) -> Top<D>
    where
        D: Doom,
    {
        let entry = Entry::new(&doom);
        self.entries.push(entry);
        Top::new(doom, self)
    }

    pub fn spot(mut self, location: (&'static str, u32)) -> Self {
        self.entries.last_mut().unwrap().set_location(location);
        self
    }

    pub(crate) fn store_original<D>(&mut self, doom: D)
    where
        D: Doom,
    {
        self.entries.last_mut().unwrap().set_original(doom);
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
        for frame in self.entries.iter().rev() {
            writeln!(f, "{frame:?}")?;
        }

        Ok(())
    }
}
