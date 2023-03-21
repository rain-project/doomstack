use crate::{Doom, Location, Stack, Top};

/// Extension methods for [`Result`] types whose [`Err`] variant is a [`Stack`] or a [`Top`].
pub trait ResultExt<O> {
    /// If `self` is an [`Err`], pushes a [`Doom`] error on top of the [`Err`]'s [`Stack`] or [`Top`].
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        D: Doom;

    /// If `self` is an [`Err`], sets the last spotting [`Location`] of the [`Err`]'s top error.
    ///
    /// See [`Stack::spot`] and [`Top::spot`] for additional information on spotting.
    fn spot(self, location: Location) -> Self;

    /// Syntax sugar for [`ResultExt::push`], then [`ResultExt::spot`].
    ///
    /// Calling `result.push(doom).spot(location)` is equivalent to calling `result.pot(doom, location)`.
    fn pot<D>(self, doom: D, location: Location) -> Result<O, Top<D>>
    where
        Self: Sized,
        D: Doom,
    {
        self.push(doom).spot(location)
    }
}

impl<O> ResultExt<O> for Result<O, Stack> {
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        D: Doom,
    {
        self.map_err(|error| error.push(doom))
    }

    fn spot(self, location: Location) -> Self {
        self.map_err(|error| error.spot(location))
    }
}

impl<O, E> ResultExt<O> for Result<O, Top<E>>
where
    E: Doom,
{
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        D: Doom,
    {
        self.map_err(|error| error.push(doom))
    }

    fn spot(self, location: Location) -> Self {
        self.map_err(|error| error.spot(location))
    }
}
