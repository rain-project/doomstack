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

    fn wrap<W, D>(self, wrap: W) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom;

    fn wrot<W, D>(self, wrap: W, location: Location) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
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

    fn wrap<W, D>(self, wrap: W) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom,
    {
        self.map_err(|error| wrap(error).into_top())
    }

    fn wrot<W, D>(self, wrap: W, location: Location) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom,
    {
        ResultExt::spot(ResultExt::wrap(self, wrap), location)
    }
}
