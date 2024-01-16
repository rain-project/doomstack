use crate::{Doom, Location, Stack, Top};

/// An interface shared by all [`Result`]s whose [`Err`] variant is a [doomstack](crate) error.
///
/// This trait is not exposed at the [`crate`]-level, and it is not meant to be user-implemented.
/// Implementations are provided for `Result<O, Stack>` and `Result<O, Top<D>>`, using the methods
/// provided by both [`Stack`]s and [`Top`]s (such as [`Stack::push`] / [`Top::push`]) to map
/// the [`Result`]'s [`Err`] variant.
///
/// [`ResultExt`] (which is publicly available) uses [`DoomResult`] to distinguish between
/// [`doomstack`](crate) [`Result`]s (for which it provides methods such as [`ResultExt::push`]) and
/// foreign [`Result`]s (for which it only offers a wrapping interface).
///
/// [`ResultExt`]: crate::ResultExt
/// [`ResultExt::push`]: crate::ResultExt::push
pub trait DoomResult<O> {
    /// Maps the [`Result`]'s [`Err`] through [`Stack::push`] / [`Top::push`]].
    fn push<P>(self, doom: P) -> Result<O, Top<P>>
    where
        P: Doom;

    /// Maps the [`Result`]'s [`Err`] through [`Stack::push_as_stack`] / [`Top::push_as_stack`].
    fn push_as_stack<P>(self, doom: P) -> Result<O, Stack>
    where
        P: Doom;

    /// Maps the [`Result`]'s [`Err`] through [`Stack::spot`] / [`Top::spot`].
    fn spot(self, location: Location) -> Self;
}

impl<O> DoomResult<O> for Result<O, Stack> {
    fn push<P>(self, doom: P) -> Result<O, Top<P>>
    where
        P: Doom,
    {
        self.map_err(|stack| stack.push(doom))
    }

    fn push_as_stack<P>(self, doom: P) -> Result<O, Stack>
    where
        P: Doom,
    {
        self.map_err(|stack| stack.push_as_stack(doom))
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

    fn push_as_stack<P>(self, doom: P) -> Result<O, Stack>
    where
        P: Doom,
    {
        self.map_err(|top| top.push_as_stack(doom))
    }

    fn spot(self, location: Location) -> Self {
        self.map_err(|top| top.spot(location))
    }
}
