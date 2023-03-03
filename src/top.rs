use crate::{Doom, Stack};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Clone)]
pub struct Top<D: Doom> {
    top: D,
    base: Stack,
}

impl<D> Top<D>
where
    D: Doom,
{
    pub(crate) fn from_parts(top: D, base: Stack) -> Self {
        Top { top, base }
    }

    pub fn top(&self) -> &D {
        &self.top
    }

    pub fn base(&self) -> &Stack {
        &self.base
    }

    pub fn push<P>(self, doom: P) -> Top<P>
    where
        P: Doom,
    {
        Stack::from(self).push(doom)
    }

    pub fn push_as_stack<P>(self, doom: P) -> Stack
    where
        P: Doom,
    {
        Stack::from(self).push_as_stack(doom)
    }

    pub fn spot(mut self, location: (&'static str, u32)) -> Self {
        self.base = self.base.spot(location);
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
    fn from(top: Top<D>) -> Self {
        let Top { top, base: stack } = top;
        stack.push_as_stack(top)
    }
}

impl<D> Error for Top<D> where D: Doom {}

impl<D> Display for Top<D>
where
    D: Doom,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.base)
    }
}

impl<D> Debug for Top<D>
where
    D: Doom,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.base)
    }
}
