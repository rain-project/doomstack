use crate::{Doom, Entry, Top};
use std::fmt::{self, Debug, Display, Formatter};

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

    pub(crate) fn store<D>(mut self, doom: D) -> Self
    where
        D: Doom,
    {
        let last = self.entries.pop().unwrap();
        self.entries.push(last.store(doom));

        self
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
        let last = self.entries.pop().unwrap();
        let last = last.spot(location);
        self.entries.push(last);

        self
    }
}

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
