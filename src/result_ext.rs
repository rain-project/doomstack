use crate::{Doom, DoomResult, Location, Stack, Top};

/// An interface extending the behavior of [`Result`] with [doomstack](crate) functionality.
///
/// # Example
///
/// ```
/// use doomstack::{here, Doom, ResultExt, Stack, Top};
///
/// struct ExternalError;
///
/// #[derive(Doom)]
/// #[doom(description("Initial error"))]
/// #[doom(wrap(initial_error))]
/// struct InitialError(ExternalError);
///
/// #[derive(Doom)]
/// #[doom(description("Intermediate error"))]
/// struct IntermediateError;
///
/// #[derive(Doom)]
/// #[doom(description("Final error"))]
/// struct FinalError;
///
/// fn externally() -> Result<(), ExternalError> {
///     // ...
///     # unimplemented!()
/// }
///
/// fn initially() -> Result<(), Top<InitialError>> {
///     externally().wrot(InitialError::initial_error, here!())?;
///     // ...
///     # unimplemented!()
/// }
///
/// fn intermediately() -> Result<(), Top<IntermediateError>> {
///     initially().pot(IntermediateError, here!())?;
///     // ...
///     # unimplemented!()
/// }
///
/// fn finally() -> Result<(), Stack> {
///     intermediately().pot_as_stack(FinalError, here!())?;
///     // ...
///     # unimplemented!()
/// }
/// ```
///
/// # Pushing, wrapping, spotting
///
/// [`Stack`]s and [`Top`]s expose methods to push new [`Doom`]s on a stack of errors, or to spot the top
/// error in a stack at a specific code [`Location`]. On top of that, [`Doom`]'s derive macro can
/// implement wrapping constructors for your [`Doom`]s, simplifying the interface between
/// [doomstack](crate) and non-[doomstack](crate) errors.
///
/// In practice, however, you will rarely handle [`Doom`]s, [`Stack`]s and [`Top`]s directly.
/// Rather, your day-to-day business will likely deal with [`Result`]s. Consider the most common
/// case possible. As in the example above, you have a function that returns a [doomstack](crate)
/// [`Result`]:
///
/// ```
/// # use doomstack::{Doom, Top};
/// #
/// # #[derive(Doom)]
/// # #[doom(description("..."))]
/// # struct WentWrong;
/// #
/// fn might_go_wrong() -> Result<u32, Top<WentWrong>> {
///     // ...
///     # unimplemented!()
/// }
/// ```
///
/// Now, you would like to invoke `might_go_wrong()` and get the [`u32`]; if something `WentWrong`,
/// you would like to [`Top::push`], say, a `FailedToGetNumber` and propagate the error. Indeed,
/// that is what the `?` operator is all about! Without any help, however, you are stuck with quite
/// a lot of boilerplate:
///
/// ```
/// # use doomstack::{Doom, Top};
/// #
/// # #[derive(Doom)]
/// # #[doom(description("..."))]
/// # struct WentWrong;
/// #
/// # #[derive(Doom)]
/// # #[doom(description("..."))]
/// # struct FailedToGetNumber;
/// #
/// # fn might_go_wrong() -> Result<u32, Top<WentWrong>> {
/// #     // ...
/// #     unimplemented!()
/// # }
/// #
/// fn do_something_with_number() -> Result<(), Top<FailedToGetNumber>> {
///     let number = might_go_wrong().map_err(|top| top.push(FailedToGetNumber))?;
///     // ...
///     # unimplemented!()
/// }
/// ```
///
/// In this and many similar cases, [`ResultExt`] is exactly what you need. The [`ResultExt`] trait
/// extends the interface of [`Result`]s with several useful methods to conditionally manipulate
/// [`Err`]s. These allow to forward calls to fundamentals such as [`Stack::push`] / [`Top::push`]
/// or [`Stack::spot`] / [`Top::spot`], as well as syntax sugar such as [`Stack::pot`] /
/// [`Top::pot`].
///
/// [`ResultExt`] is useful also when dealing with non-[doomstack](crate) errors. When provided with
/// an appropriate wrapping constructor (see [`Doom`]'s derive macro for additional detail),
/// [`ResultExt::wrap`] / [`ResultExt::wrap_as_stack`] map the [`Err`] through the wrapping
/// constructor, then wrap the resulting [`Doom`] into a [`Top`] / [`Stack`]. Presto! With one
/// simple call, your [`Result`] is [doomstack](crate) compatible.

pub trait ResultExt<O, E> {
    /// Transforms the [`Result`]'s [`Err`] by conditionally forwarding `doom` to the error's
    /// [`Stack::push`] / [`Top::push`] method.
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        Self: DoomResult<O>,
        D: Doom;

    /// Transforms the [`Result`]'s [`Err`] by conditionally forwarding `doom` to the error's
    /// [`Stack::push_as_stack`] / [`Top::push_as_stack`] method.
    fn push_as_stack<D>(self, doom: D) -> Result<O, Stack>
    where
        Self: DoomResult<O>,
        D: Doom;

    /// Transforms the [`Result`]'s [`Err`] by conditionally mapping the error through `wrap`,
    /// then [`Doom::into_top`].
    fn wrap<W, D>(self, wrap: W) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom;

    /// Transforms the [`Result`]'s [`Err`] by conditionally mapping the error through `wrap`,
    /// then [`Doom::into_stack`].
    fn wrap_as_stack<W, D>(self, wrap: W) -> Result<O, Stack>
    where
        W: Fn(E) -> D,
        D: Doom;

    /// Conditionally invokes [`Stack::spot`] / [`Top::spot`] on the [`Result`]'s error.
    fn spot(self, location: Location) -> Self
    where
        Self: DoomResult<O>;

    /// Syntax sugar for [`ResultExt::push`], then [`ResultExt::spot`].
    ///
    /// Calling `result.pot(doom, location)` is equivalent to calling
    /// `result.push(doom).spot(location)`.
    fn pot<D>(self, doom: D, location: Location) -> Result<O, Top<D>>
    where
        Self: DoomResult<O>,
        D: Doom;

    /// Syntax sugar for [`ResultExt::push_as_stack`], then [`ResultExt::spot`].
    ///
    /// Calling `result.pot_as_stack(doom, location)` is equivalent to calling
    /// `result.push_as_stack(doom).spot(location)`.
    fn pot_as_stack<D>(self, doom: D, location: Location) -> Result<O, Stack>
    where
        Self: DoomResult<O>,
        D: Doom;

    /// Syntax sugar for [`ResultExt::wrap`], then [`ResultExt::spot`].
    ///
    /// Calling `result.wrot(doom, location)` is equivalent to calling
    /// `result.wrap(wrap).spot(location)`.
    fn wrot<W, D>(self, wrap: W, location: Location) -> Result<O, Top<D>>
    where
        W: Fn(E) -> D,
        D: Doom;

    /// Syntax sugar for [`ResultExt::wrap_as_stack`], then [`ResultExt::spot`].
    ///
    /// Calling `result.wrot_as_stack(doom, location)` is equivalent to calling
    /// `result.wrap_as_stack(wrap).spot(location)`.
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
