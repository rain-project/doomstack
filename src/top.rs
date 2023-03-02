use crate::{Doom, Entry, Stack};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Clone)]
pub struct Top<D: Doom> {
    top: D,
    stack: Stack,
}

impl<D> Top<D>
where
    D: Doom,
{
    pub(crate) fn new(top: D, stack: Stack) -> Self {
        Top { top, stack }
    }

    pub fn top(&self) -> &D {
        &self.top
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn entries(&self) -> &[Entry] {
        self.stack.entries()
    }

    pub fn push<P>(self, doom: P) -> Top<P>
    where
        P: Doom,
    {
        let stack = Stack::from(self);
        stack.push(doom)
    }

    pub fn spot(mut self, location: (&'static str, u32)) -> Self {
        self.stack = self.stack.spot(location);
        self
    }

    pub fn pot<P>(self, doom: P, location: (&'static str, u32)) -> Top<P>
    where
        P: Doom,
    {
        self.push(doom).spot(location)
    }
}

impl<D> From<Top<D>> for Stack
where
    D: Doom,
{
    fn from(mut top: Top<D>) -> Self {
        if D::keep_original() {
            top.stack.store_original(top.top);
        }

        top.stack
    }
}

impl<D> Error for Top<D> where D: Doom {}

impl<D> Display for Top<D>
where
    D: Doom,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.stack)
    }
}

impl<D> Debug for Top<D>
where
    D: Doom,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.stack)
    }
}
