use crate::{Doom, Location, Stack};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// A [`Doom`] error on top of a [`Stack`] of [`Entry`]-ies.
///
/// # Example
///
/// #### Snippet
///
/// ```
/// use doomstack::{here, Doom, Top};
///
/// #[derive(Doom)]
/// enum ToolError {
///     #[doom(description("Rake lost all its teeth"))]
///     RakeToothless,
///     #[doom(description("Hose stolen by neighbor"))]
///     HoseStolen {
///         neighbor_location: NeighborLocation,
///     },
/// }
///
/// enum NeighborLocation {
///     Home,
///     Work,
///     Unknown,
/// }
///
/// fn tend_garden(rake_has_teeth: bool, hose_still_there: bool, neighbor_location: NeighborLocation) -> Result<(), Top<ToolError>> {
///     if !rake_has_teeth {
///         return ToolError::RakeToothless.fail();
///     }
///
///     if !hose_still_there {
///         return ToolError::HoseStolen {neighbor_location}.fail();
///     }
///
///     Ok(())
/// }
///
/// #[derive(Doom)]
/// enum GardeningError {
///     #[doom(description("Failed to tend garden"))]
///     TendFailed,
///     #[doom(description("Sun is no longer in the sky"))]
///     SunDisappeared,
/// }
///
/// fn hunt_neighbor_down() {
///     // ...
/// }
///
/// fn grow_plants(
///     rake_has_teeth: bool,
///     hose_still_there: bool,
///     neighbor_location: NeighborLocation,
///     sun_still_there: bool,
/// ) -> Result<(), Top<GardeningError>> {
///     if let Err(tool_top) = tend_garden(rake_has_teeth, hose_still_there, neighbor_location) {
///         match tool_top.doom() {
///             ToolError::HoseStolen { neighbor_location: NeighborLocation::Home | NeighborLocation::Work } => {
///                 hunt_neighbor_down();
///             },
///             _ => {
///                 let gardening_top = tool_top.push(GardeningError::TendFailed);
///                 return Err(gardening_top.spot(here!()));
///             }
///         }
///     }
///     
///     if !sun_still_there {
///         return GardeningError::SunDisappeared.fail();
///     }
///
///     Ok(())
/// }
///
/// ```
///
/// #### Discussion
///
/// _(This example is a more articulated version of the one you can find in our [`crate`]-level
/// introduction to Doomstack. If you feel this discussion is too high-level, check out that
/// example as a warm-up.)_
///
/// To `grow_plants()`, we must first `tend_garden()`, then hope that `sun_still_there`! To
/// keep track of everything that could go wrong, we define two enums: `ToolError` for
/// `tend_garden()`, `GardeningError` for `grow_plants()`. Both enums implement [`Doom`],
/// using variants to encode the various error conditions.
///
/// As often happens, we know how to handle programmatically some, but not all, error conditions.
/// If our hose has been stolen (`ToolError::HoseStolen`), and we know where to find our
/// sticky-fingered neighbor (`NeighborLocation::Home` or `NeighborLocation::Work`), then
/// `hunt_neighbor_down()` is a guaranteed fix for our problem. In all other cases, we are
/// out of luck (try finding an easy fix for `GardeningError::SunDisappeared`!) - all
/// we can do is propagate the error to the caller, let them decide what to do.
///
/// Lucky us, programmatically handling recently occurred errors is exactly what [`Top`]s are
/// designed for. Let's have a look at `grow_plants()`. If `tend_garden()` returns an
/// `Err(tool_top)`, we want to `match` the top, most recent error in `tool_top` to determine
/// if there is something we can do. This is easily done! Remember, `tool_top` is a
/// `Top<ToolError>`. This means `tool_top` stores (without any need for heap allocations or
/// dynamic dispatch) a full, typed copy of the `ToolError` at the very top of its stack.
/// All we need to do is use `tool_top`'s [`Top::doom`] getter to obtain a reference to the
/// `ToolError`, ready for us to `match`. If `ToolError::HoseStolen`, and `neighbor_location`
/// is `NeighborLocation::Home` or `NeighborLocation::Work`, then we `hunt_neighbor_down()`
/// and we are back on track. If the neighbor is at large, `grow_plants()` cannot proceed and
/// we must return a `Top<GardeningError>` indicating why. To do so, we use [`Top::push`] to
/// push a `GardeningError::TendFailed` on top of `tool_top`:
///
///  - The `ToolError` at the top of `tool_top` is archived in an [`Entry`], storing the
///    `ToolError`'s [`Doom::tag`] and [`Doom::description`] as text.
///  - The `GardeningError` becomes the new top, most recent element of the error stack.
///
/// By [`Top::push`]-ing a `GardeningError` on top of our `Top<ToolError>`, we produce a new
/// `Top<GardeningError>` `gardening_top` stacking two errors: a fully typed
/// `GardeningError::TendFailed`, on top of a `ToolError`, now archived in an [`Entry`].
///
/// Before returning `gardening_top`, we [`Top::spot`] it [`here!()`] for good measure.
/// In doing so, we tag `gardening_top`'s top error (the `GardeningError`) as having
/// occurred [`here!()`].
///
/// # The difference between [`Stack`]s and [`Top`]s
///
/// Both [`Stack`]s and [`Top`]s repesent a stack of [`Doom`] errors (most recent on top),
/// but they differ in how they store the top error.
///
/// ##### [`Stack`]s
///
/// All errors in a [`Stack`] are stored in their archived form as [`Entry`]-ies. As
/// such, [`Stack`] is a concrete (not generic) type. Each [`Entry`] of a [`Stack`]
/// captures its error's [`Doom::tag()`] and [`Doom::description()`] as text, storing
/// the error itself (which might have, e.g., variants or fields that are useful for
/// programmatic error-handling) only if requested by [`Doom::keep_original()`] upon
/// archival. In that case, the error is stored in its original form by its [`Entry`]
/// (but this comes at a cost in terms of heap allocation and dynamic dispatch, see
/// [`Entry`] for additional details).
///
/// ##### [`Top`]s
///
/// Unlike a [`Stack`], a [`Top`] is a generic type, and as such it can store its top,
/// most recent error in its original form (allowing, e.g., direct access to whatever
/// useful variants or fields it might have) without the need for heap allocation or
/// dynamic dispatch. A [`Top<D>`] stores an error of type `D` on top of a [`Stack`]
/// of (zero or more) [`Entry`]-ies archiving `D`'s predecessors. Whenever you come
/// across a `Top<D>`, think:
///
/// > _"A `D` just happened, possibly resulting from previous errors lower in the stack."_
///
/// By design, [`Top<D>`] strikes a compromise, allowing cheap, stack-based access to
/// its top error, at the cost of [`Top`] being generic.
///
/// _(See [Design philosophy](crate#design-philosophy) for a discussion on the balance [`Top`]s and
/// [`Stack`]s aim to strike between typed and non-typed error handling.)_
///
/// [`Entry`]: crate::Entry
/// [`here!()`]: crate::here!
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
    /// Creates a `Top` from a `Doom` and a `Stack`.
    pub(crate) fn from_parts(doom: D, stack: Stack) -> Self {
        Top {
            doom,
            location: None,
            stack,
        }
    }

    /// Returns a reference to the [`Top`]'s top error.
    ///
    /// `Top<D>` stores its top (most recent) error as a concrete `D` instance. [`Top::doom`]
    /// returns a reference to that error.
    pub fn doom(&self) -> &D {
        &self.doom
    }

    /// Returns the (optional) [`Location`] at which `self` was last [`spot()`]-ted.
    ///
    /// [`spot()`]: Top::spot
    pub fn location(&self) -> Option<Location> {
        self.location
    }

    /// Returns a reference to the [`Stack`] of errors that precede `self`'s top error.
    ///
    /// [`Top`] stores all but the top error in their archived form, as [`Entry`]-ies in a
    /// [`Stack`]. [`Top::stack`] returns a reference to that [`Stack`].
    ///
    /// [`Entry`]: crate::Entry
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    /// Pushes a [`Doom`] error on top of the current [`Top`].
    ///
    /// The resulting [`Top`] stores the new error as-is: this is useful, e.g., if the error
    /// being pushed contains variants or fields that are useful for error handling. The error
    /// at the top of `self`, which is no longer at the top, is archived in an [`Entry`].
    ///
    /// [`Entry`]: crate::Entry
    /// [`push()`]: Top::push
    /// [`stack`]: Top::stack
    pub fn push<P>(self, doom: P) -> Top<P>
    where
        P: Doom,
    {
        Stack::from(self).push(doom)
    }

    /// Pushes a [`Doom`] error on top of the current [`Top`], producing a [`Stack`].
    ///
    /// The resulting [`Stack`] stores all errors as [`Entry`]-ies, in their archived form.
    /// This means that `doom` is immediately archived (if [`Top::push`] is used instead,
    /// doom is stored as-is instead).
    ///
    /// [`Entry`]: crate::Entry
    pub fn push_as_stack<P>(self, doom: P) -> Stack
    where
        P: Doom,
    {
        Stack::from(self).push_as_stack(doom)
    }

    /// Sets the last spotting [`Location`] for the top error in the [`Top`].
    ///
    /// If called multiple times, [`Top::spot`] simply overwrites the last spotting
    /// [`Location`]. To track error propagation, consider [`Top::push`]-ing more
    /// [`Doom`] errors, [`Top::spot`]-ing each at a relevant [`Location`].
    pub fn spot(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    /// Syntax sugar for [`Top::push`], then [`Top::spot`].
    ///
    /// Calling `top.pot(doom, location)` is equivalent to calling `top.push(doom).spot(location)`.
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
