use crate::{Doom, Entry, Location, Top};
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};

/// A stack of [`Entry`]-ies, each archiving a [`Doom`] error.
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
/// terms of heap allocation and dynamic typing (see [`Entry`] for additional details).
///
/// Unlike a [`Stack`], a [`Top`] is a generic type, and as such it can store its top
/// error in its original form (along with whatever useful fields it might have) without
/// the need for heap allocation or dynamic dispatching. A [`Top<D>`] stores an error of
/// type `D` on top of a [`Stack`] of (zero or more) [`Entry`]-ies archiving older errors.
/// As such, [`Top<D>`] strikes a compromise, allowing cheap, stack-based access to its
/// top error, at the cost of [`Top`] being a generic type: as such, [`Top`]s are not
/// interchangeable in general.
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
/// # Examples
///
/// ```
/// use doomstack::{Description, Doom, Stack};
///
/// struct AnError;
///
/// struct AnotherError {
///     severity: u32
/// };
///
/// impl Doom for AnError {
///     fn tag(&self) -> &'static str {
///         "AnError"
///     }
///
///     fn description(&self) -> Description {
///         Description::Static("An error occurred")
///     }
/// }
///
/// impl Doom for AnotherError {
///     fn tag(&self) -> &'static str {
///         "AnotherError"
///     }
///
///     fn description(&self) -> Description {
///         Description::Owned(format!("Another error occurred, with severity {}", self.severity))
///     }
/// }
///
/// let an_error = AnError;
/// let stack = an_error.into_stack();
///
/// assert_eq!(stack.entries().count(), 1);
/// assert_eq!(stack.entries().next().unwrap().tag(), "AnError");
///
/// let another_error = AnotherError { severity: 3 };
/// let top = stack.push(another_error);
///
/// assert_eq!(top.doom().severity, 3); // `top` stores `another_error`
///
/// let stack = Stack::from(top);
///
/// assert_eq!(stack.entries().count(), 2);
///
/// // `stack` no longer stores `another_error`: `severity` is no longer retrievable,
/// // but `another_error`'s personalized description is still there:
///
/// assert_eq!(
///     stack.entries().next().unwrap().description(),
///     "Another error occurred, with severity 3"
/// );
///
/// let yet_another_error = AnotherError { severity: 7 };
/// let stack = stack.push_as_stack(yet_another_error);
///
/// // `stack` does not store `yet_another_error` either.
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

    /// Syntax sugar for [`Stack::push`], then [`Stack::spot`].
    ///
    /// Calling `stack.push(doom).spot(location)` is equivalent to calling `stack.pot(doom, location)`.
    pub fn pot<P>(self, doom: P, location: Location) -> Top<P>
    where
        P: Doom,
    {
        self.push(doom).spot(location)
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
