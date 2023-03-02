use crate::{Doom, Entry, Stack};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Clone)]
pub struct Top<T: Doom> {
    top: T,
    stack: Stack,
}

impl<T> Top<T>
where
    T: Doom,
{
    pub(crate) fn new(top: T, stack: Stack) -> Self {
        Top { top, stack }
    }

    pub fn top(&self) -> &T {
        &self.top
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn entries(&self) -> &[Entry] {
        self.stack.entries()
    }

    pub fn push<D>(self, doom: D) -> Top<D>
    where
        D: Doom,
    {
        let stack = Stack::from(self);
        stack.push(doom)
    }

    pub fn spot(mut self, location: (&'static str, u32)) -> Self {
        self.stack = self.stack.spot(location);
        self
    }

    pub fn pot<S>(self, doom: S, location: (&'static str, u32)) -> Top<S>
    where
        S: Doom,
    {
        self.push(doom).spot(location)
    }
}

impl<T> From<Top<T>> for Stack
where
    T: Doom,
{
    fn from(top: Top<T>) -> Self {
        if T::store() {
            top.stack.store(top.top)
        } else {
            top.stack
        }
    }
}

impl<T> Error for Top<T> where T: Doom {}

impl<T> Display for Top<T>
where
    T: Doom,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.stack)
    }
}

impl<T> Debug for Top<T>
where
    T: Doom,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.stack)
    }
}
