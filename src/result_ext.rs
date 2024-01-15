use crate::{Doom, DoomResult, Location, Stack, Top};

pub trait ResultExt<O, E> {
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        Self: DoomResult<O>,
        D: Doom;

    fn push_as_stack<D>(self, doom: D) -> Result<O, Stack>
    where
        Self: DoomResult<O>,
        D: Doom;

    fn wrap<W, D>(self, wrap: W) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom;

    fn wrap_as_stack<W, D>(self, wrap: W) -> Result<O, Stack>
    where
        W: Fn(E) -> D,
        D: Doom;

    fn spot(self, location: Location) -> Self
    where
        Self: DoomResult<O>;

    fn pot<D>(self, doom: D, location: Location) -> Result<O, Top<D>>
    where
        Self: DoomResult<O>,
        D: Doom;

    fn pot_as_stack<D>(self, doom: D, location: Location) -> Result<O, Stack>
    where
        Self: DoomResult<O>,
        D: Doom;

    fn wrot<W, D>(self, wrap: W, location: Location) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom;

    fn wrot_as_stack<W, D>(self, wrap: W, location: Location) -> Result<O, Stack>
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

    fn push_as_stack<D>(self, doom: D) -> Result<O, Stack>
    where
        Self: DoomResult<O>,
        D: Doom,
    {
        DoomResult::push_as_stack(self, doom)
    }

    fn wrap<W, D>(self, wrap: W) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom,
    {
        self.map_err(|error| wrap(error).into_top())
    }

    fn wrap_as_stack<W, D>(self, wrap: W) -> Result<O, Stack>
    where
        W: Fn(E) -> D,
        D: Doom,
    {
        self.map_err(|error| wrap(error).into_stack())
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

    fn pot_as_stack<D>(self, doom: D, location: Location) -> Result<O, Stack>
    where
        Self: DoomResult<O>,
        D: Doom,
    {
        ResultExt::spot(ResultExt::push_as_stack(self, doom), location)
    }

    fn wrot<W, D>(self, wrap: W, location: Location) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom,
    {
        ResultExt::spot(ResultExt::wrap(self, wrap), location)
    }

    fn wrot_as_stack<W, D>(self, wrap: W, location: Location) -> Result<O, Stack>
    where
        W: Fn(E) -> D,
        D: Doom,
    {
        ResultExt::spot(ResultExt::wrap_as_stack(self, wrap), location)
    }
}
