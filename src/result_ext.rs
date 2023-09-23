use crate::{Doom, DoomResult, Location, Top};

pub trait ResultExt<O, E> {
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        Self: DoomResult<O>,
        D: Doom;

    fn spot(self, location: Location) -> Self
    where
        Self: DoomResult<O>;

    fn pot<D>(self, doom: D, location: Location) -> Result<O, Top<D>>
    where
        Self: DoomResult<O>,
        D: Doom;
}

impl<O, E> ResultExt<O, E> for Result<O, E> {
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        Self: DoomResult<O>,
        D: Doom,
    {
        DoomResult::push(self, doom)
    }

    fn spot(self, location: Location) -> Self
    where
        Self: DoomResult<O>,
    {
        DoomResult::spot(self, location)
    }

    fn pot<D>(self, doom: D, location: Location) -> Result<O, Top<D>>
    where
        Self: DoomResult<O>,
        D: Doom,
    {
        ResultExt::spot(ResultExt::push(self, doom), location)
    }
}
