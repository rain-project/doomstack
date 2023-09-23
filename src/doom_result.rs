use crate::{Doom, Location, Stack, Top};

pub trait DoomResult<O> {
    fn push<P>(self, doom: P) -> Result<O, Top<P>>
    where
        P: Doom;

    fn spot(self, location: Location) -> Self;
}

impl<O> DoomResult<O> for Result<O, Stack> {
    fn push<P>(self, doom: P) -> Result<O, Top<P>>
    where
        P: Doom,
    {
        self.map_err(|stack| stack.push(doom))
    }

    fn spot(self, location: Location) -> Self {
        self.map_err(|stack| stack.spot(location))
    }
}

impl<O, D> DoomResult<O> for Result<O, Top<D>>
where
    D: Doom,
{
    fn push<P>(self, doom: P) -> Result<O, Top<P>>
    where
        P: Doom,
    {
        self.map_err(|top| top.push(doom))
    }

    fn spot(self, location: Location) -> Self {
        self.map_err(|top| top.spot(location))
    }
}
