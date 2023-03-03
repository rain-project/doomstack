use crate::{Doom, Location, Stack};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Clone)]
pub struct Top<D: Doom> {
    doom: D,
    location: Option<Location>,
    stack: Stack,
}

impl<D> Top<D>
where
    D: Doom,
{
    pub(crate) fn from_parts(doom: D, stack: Stack) -> Self {
        Top {
            doom,
            location: None,
            stack,
        }
    }

    pub fn doom(&self) -> &D {
        &self.doom
    }

    pub fn location(&self) -> Option<Location> {
        self.location
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
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

    pub fn spot(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    pub fn pot<P>(self, doom: P, location: Location) -> Top<P>
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
        let Top {
            doom: top,
            location,
            stack,
        } = top;

        let stack = stack.push_as_stack(top);

        if let Some(location) = location {
            stack.spot(location)
        } else {
            stack
        }
    }
}

impl<D> Error for Top<D> where D: Doom {}

impl<D> Display for Top<D>
where
    D: Doom,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "<top: {}>", self.doom.tag())
    }
}

impl<D> Debug for Top<D>
where
    D: Doom,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(location) = self.location {
            writeln!(
                f,
                "[{} @ {}] {}",
                self.doom.tag(),
                location,
                self.doom.description()
            )?;
        } else {
            writeln!(f, "[{}] {}", self.doom.tag(), self.doom.description())?;
        }

        write!(f, "{:?}", self.stack)
    }
}
