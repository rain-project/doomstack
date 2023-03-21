use crate::{Doom, Location, Stack};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// A [`Doom`] error on top of a [`Stack`] of [`Entry`]-ies.
/// 
/// # The difference between [`Stack`]s and [`Top`]s
///
/// Both [`Stack`]s and [`Top`]s repesent a stack of [`Doom`] errors (most recent on top),
/// but they differ in how they store the top error.
///
/// All errors in a [`Stack`] are stored in their archived form as [`Entry`]-ies. As
/// such, [`Stack`] is a concrete (not generic) type. Each [`Entry`] of a [`Stack`]
/// captures the text-based [`Doom::tag()`] and [`Doom::description()`] for its error,
/// storing the error itself (which might have, e.g., fields that are useful for later
/// error-handling) only if requested by [`Doom::keep_original()`]. In that case, the
/// error is stored in its original form by its [`Entry`], but this comes at a cost in
/// terms of heap allocation and dynamic dispatch (see [`Entry`] for additional details).
///
/// Unlike a [`Stack`], a [`Top`] is a generic type, and as such it can store its top
/// error in its original form (allowing, e.g., direct access to whatever useful fields 
/// it might have) without the need for heap allocation or dynamic dispatching. A 
/// [`Top<D>`] stores an error of type `D` on top of a [`Stack`] of (zero or more) 
/// [`Entry`]-ies archiving `D`'s predecessors. As such, [`Top<D>`] strikes a compromise, 
/// allowing cheap, stack-based access to its top error, at the cost of [`Top`] being a 
/// generic type: as such, [`Top`]s are not interchangeable in general.
/// 
/// # Converting and pushing
///
/// A [`Top`] can be converted into a [`Stack`] ([`Stack`] implements `From<Top<D>>`
/// for any `D: Doom`). Upon conversion, the top error of the [`Top`] (whose concrete
/// instance the [`Top`] stores) is archived in an [`Entry`] and pushed on top of the
/// resulting [`Stack`]. As the conversion from [`Top`] to [`Stack`] is lossy in
/// general, a [`Stack`] cannot be converted back into a [`Top`].
///
/// A new [`Doom`] error can be pushed on top of a [`Stack`] by one of two means (see
/// [`crate`]-level documentation):
///  - By invoking [`Stack::push`]. The new error is stored as-is in a [`Top`], on top
///    of the current [`Stack`] of [`Entry`]-ies.
///  - By invoking [`Stack::push_as_stack`]. The error is archived in an [`Entry`],
///    which is placed at the top of the [`Stack`]. This is equivalent to invoking
///    [`Stack::push`], then [`Stack::from`] on the result.
/// 
/// [`Entry`]: crate::Entry
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

    /// Returns the (optional) [`Location`] at which `self` was last `spot()`-ted.
    /// 
    /// [`spot()`]: Top::spot
    pub fn location(&self) -> Option<Location> {
        self.location
    }

    /// Returns a reference to the [`Stack`] of `self`'s predecessor.
    /// 
    /// [`Top`] stores all but the top error in their archived form, as [`Entry`]-ies in a
    /// [`Stack`]. [`Top::stack`] returns a reference to that [`Stack`].
    /// 
    /// [`Entry`]: crate::Entry
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    /// Pushes a [`Doom`] error on top of the current [`Top`], producing a new [`Top`].
    /// 
    /// The resulting [`Top`] stores the new error as-is: this is useful, e.g., if the error 
    /// being pushed contains fields that are useful for error handling. Note that, upon 
    /// [`push()`]-ing, `self`'s top error is archived in an [`Entry`], which is pushed
    /// on the top of the new [`Top`]'s [`stack`].
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
    /// The resulting [`Stack`] stores the new error as an [`Entry`], in its archived form.
    /// 
    /// [`Entry`]: crate::Entry
    pub fn push_as_stack<P>(self, doom: P) -> Stack
    where
        P: Doom,
    {
        Stack::from(self).push_as_stack(doom)
    }

    /// Sets the last spotting [`Location`] for the top error in the [`Top`].
    pub fn spot(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }

    /// Syntax sugar for [`Top::push`], then [`Top::spot`].
    /// 
    /// Calling `top.push(doom).spot(location)` is equivalent to calling `top.pot(doom, location)`.
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
