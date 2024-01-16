use crate::{Doom, Entry, Location, Top};
use std::{
    any::Any,
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// A stack of [`Entry`]-ies, each archiving a [`Doom`] error.
///
/// _(For the difference between [`Stack`]s and [`Top`]s, please refer to [`Top`]'s
/// documentation.)_
///
/// # Example
///
/// #### Snippet
///
/// ```
/// use doomstack::{prelude::*, Stack};
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
///
/// fn garden(hungry_birds: bool, humidity: f32, napalm_leak: bool) -> Result<(), Stack> {
///     seed(hungry_birds, humidity).push_as_stack(GardeningError::SeedFailed)?;
///
///     if napalm_leak {
///         return GardeningError::GardenCaughtFire.fail_as_stack();
///     }
///
///     Ok(())
/// }
/// ```
///
/// #### Discussion
///
/// _(This example follows the general structure of the one you can find in [`Top`]'s documentation.
/// If you are interested in a more in-depth discussion about error handling in [doomstack](crate),
/// check out that example for additional information.)_
///
/// Similarly to many other examples in this documentation, here we `garden()`, and by that we mean,
/// we `seed()` our garden, then hope a `napalm_leak` doesn't accidentally set our plants on fire.
/// To keep track of everthing that could go wrong, we define two enums: `SeedError` for `seed()`,
/// `GardeningError` for `garden()`. Both enums implement [`Doom`], using variants to encode the
/// various error conditions.
///
/// Unlike what happens when [`Top`]s are involved, however, here the signature is the same for both
/// `seed()` and `garden()`: `Result<(), Stack>`. To accommodate this, we [`Doom::fail_as_stack`]
/// our [`Doom`]s (as opposed to [`Doom::fail`]-ing them), and we [`ResultExt::push_as_stack`] on
/// our [`Result`]s (instead of [`ResultExt::push`]-ing).
///
/// # [`Stack`]'s pros and cons
///
/// As we have seen in the example above, [`Stack`]s enable uniformly-typed error handling:
/// no matter their original type, any sequence of [`Doom`]s can be organized into a [`Stack`].
/// This can be useful to collect heterogeneous errors in one common data structure (e.g., for
/// logging purposes), or to keep a simple interface for traits that return errors.
///
/// The simplicity of [`Stack`]s, however, comes at a cost. Every time a [`Doom`] is stored in /
/// pushed on a [`Stack`], it is archived in an [`Entry`]: the [`Doom`]'s tag and description are
/// stored as stored as text; a copy of the original [`Doom`] is retained only if prescribed by
/// [`Doom::keep_original`] (in that case, the [`Doom`] is stored in a [`Box<dyn Any>`], and will
/// require dynamic dispatch to access).
///
/// Let's go back to our snippet above. What happens if we invoke `garden(false, 70., false)`? We
/// get back a [`Stack`] archiving two [`Doom`]s: a `GardeningError::SeedFailed` on top, a
/// `SeedError::SeedRotten` on the bottom. Both are stored as [`Entry`]-ies; because neither
/// [`Doom`] overrides the default value of [`Doom::keep_original`], the original [`Doom`]s are
/// dropped upon archival. If we print the bottom [`Entry`], we get
/// ```text
/// [SeedError::SeedRotten] Seed rotten (humidity was 70)
/// ```
/// The description is there, but concrete the `humidity` value is gone! Other than parsing the
/// [`Entry::description`] (_please don't do that!!_) there is not much we can do to handle
/// the `SeedError` programmatically.
///
/// **In summary**: if you are in a position to choose, use [`Top`]s. They provide zero-cost typed
/// error handling for the top, most recent error. In a pinch, [`Stack`]s will provide a uniform,
/// non-typed interface, at the cost of less-than-straightforward programmatic error handling. If
/// all you need to do is to print out your errors for a human to read, [`Top`]s and [`Stack`]s are
/// completely equivalent.
///
/// [`ResultExt::push`]: crate::ResultExt::push
/// [`ResultExt::push_as_stack`]: crate::ResultExt::push_as_stack
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

    /// Pushes a [`Doom`] error on top of the current [`Stack`], producing a [`Stack`].
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
    /// Calling `stack.pot(doom, location)` is equivalent to calling
    /// `stack.push(doom).spot(location)`.
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
