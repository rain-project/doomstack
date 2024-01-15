use crate::{Doom, Entry, Location, Top};
use std::{
    any::Any,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// A stack of [`Entry`]-ies, each archiving a [`Doom`] error.
///
/// _For the difference between [`Stack`]s and [`Top`]s, please refer to [`Top`]'s
/// documentation._
///
/// # Example
///
/// ```
/// use doomstack::{Description, Doom, Stack};
///
/// #[derive(Doom)]
/// enum SeedError {
///     #[doom(description("Seed was pecked by a bird"))]
///     SeedPecked,
///     #[doom(description("Seed rotten (humidity was {humidity})"))]
///     SeedRotten { humidity: f32 },
/// }
///
/// #[derive(Doom)]
/// enum GardeningError {
///     #[doom(description("Failed to plant seed"))]
///     SeedFailed,
///     #[doom(description("Garden caught fire"))]
///     GardenCaughtFire
/// }
///
/// fn seed(hungry_birds: bool, humidity: f32) -> Result<(), Stack> {
///     if hungry_birds {
///         return SeedError::SeedPecked.fail_as_stack();
///     }
///     
///     if humidity > 50. {
///         return SeedError::SeedRotten { humidity }.fail_as_stack();
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Default, Clone)]
pub struct Stack {
    entries: Vec<Entry>,
}

impl Stack {
    /// Constructs a new, empty [`Stack`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns an iterator over the [`Stack`]'s [`Entry`]-ies, top (i.e., most recently
    /// pushed) to bottom (i.e., first pushed).
    pub fn entries(&self) -> impl Iterator<Item = &Entry> {
        self.entries.iter().rev()
    }

    /// Returns an iterator over the [`Doom`] errors that were stored upon archival
    /// in the [`Stack`]'s [`Entry`]-ies, top (i.e., most recently pushed) to bottom
    /// (i.e., first pushed).
    pub fn originals(&self) -> impl Iterator<Item = &(dyn Any + Send + Sync)> {
        self.entries().filter_map(Entry::original)
    }

    /// Returns an iterator over the [`Doom`] errors of type `O` that were stored
    /// upon archival in the [`Stack`]'s [`Entry`]-ies, top (i.e., most recently
    /// pushed) to bottom (i.e., first pushed).
    pub fn originals_by_type<O>(&self) -> impl Iterator<Item = &O>
    where
        O: 'static,
    {
        self.originals()
            .filter_map(<dyn Any + Send + Sync>::downcast_ref::<O>)
    }

    /// Pushes a [`Doom`] error on top of the current [`Stack`], producing a [`Top`].
    ///
    /// The resulting [`Top`] stores the new error as-is: this is useful, e.g., if the
    /// error being pushed contains fields that are useful for error handling.
    pub fn push<D>(self, doom: D) -> Top<D>
    where
        D: Doom,
    {
        Top::from_parts(doom, self)
    }

    /// Pushes a [`Doom`] error on top of the current [`Stack`], producing a new [`Stack`].
    ///
    /// The resulting [`Stack`] stores the new error as an [`Entry`], in its archived form.
    pub fn push_as_stack<D>(mut self, doom: D) -> Self
    where
        D: Doom,
    {
        self.entries.push(Entry::archive(doom));
        self
    }

    /// Sets the last spotting [`Location`] for the *top* [`Entry`] in the [`Stack`].
    pub fn spot(mut self, location: Location) -> Self {
        self.entries.last_mut().unwrap().spot(location);
        self
    }

    /// Syntax sugar for [`Stack::push`], then [`Top::spot`].
    ///
    /// Calling `stack.push(doom).spot(location)` is equivalent to calling `stack.pot(doom, location)`.
    pub fn pot<P>(self, doom: P, location: Location) -> Top<P>
    where
        P: Doom,
    {
        self.push(doom).spot(location)
    }

    /// Syntax sugar for [`Stack::push_as_stack`], then [`Stack::spot`].
    ///
    /// Calling `stack.pot_as_stack(doom, location)` is equivalent to calling
    /// `stack.push_as_stack(doom).spot(location)`.
    pub fn pot_as_stack<P>(self, doom: P, location: Location) -> Stack
    where
        P: Doom,
    {
        self.push_as_stack(doom).spot(location)
    }
}

impl Error for Stack {}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "<top: {}>", self.entries.last().unwrap())
    }
}

impl Debug for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for entry in self.entries() {
            writeln!(f, "{entry:?}")?;
        }

        Ok(())
    }
}
