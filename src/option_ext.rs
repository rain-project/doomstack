use crate::{Doom, Location, ResultExt, Stack, Top};
use std::future::{self, Future};

/// An interface extending the behavior of [`Option`] with [doomstack](crate) funcionality.
///
/// # Example
///
/// ```
/// use doomstack::prelude::*;
///
/// #[derive(Doom)]
/// #[doom(description("Missing value"))]
/// struct MissingValue;
///
/// fn count_chars(value: Option<String>) -> Result<usize, Top<MissingValue>> {
///     let value = value.or_tot(MissingValue, here!())?;
///     Ok(value.len())
/// }
/// ```
///
/// # Transforming [`None`]s
///
/// [`OptionExt`] offers methods to easily transform an [`Option<T>`] into a [`Result<T, Top<D>>`].
/// [`OptionExt::or_top`] is similar to [`Option::ok_or`]. It maps [`Some(value)`] to [`Ok(value)`]
/// and [`None`] to a [`Top`] wrapping the provided [`Doom`]. Similarly [`OptionExt::or_stack`]
/// transforms an [`Option<T>`] into a [`Result<T, Stack>`]. [`OptionExt::or_tot`] /
/// [`OptionExt::or_stot`] are syntax sugar for [`OptionExt::or_top`] / [`OptionExt::or_stack`],
/// followed by [`ResultExt::spot`].
///
/// _(See [`Doom::into_top`], [`Doom::into_stack`] for additional information.)
///
/// # Hanging on [`None`]
///
/// Similarly to [`ResultExt`], [`OptionExt::unwrap_or_hang`] and [`OptionExt::expect_or_hang`]
/// allow you to hang indefinitely on a [`None`] value.
///
/// _(For more information why this might be useful, see [`ResultExt`].)_
///
/// [`Ok(value)`]: Ok
/// [`Some(value)`]: Some
pub trait OptionExt<T> {
    /// Transforms `self` into a [`Result<T, Top<D>>`]: [`Some(value)`] is mapped to [`Ok(value)`],
    /// [`None`] is mapped to a [`Top`] wrapping `doom`.
    ///
    /// _(See [`Doom::into_top`] for additional information.)_
    ///
    /// [`Ok(value)`]: Ok
    /// [`Some(value)`]: Some
    fn or_top<D>(self, doom: D) -> Result<T, Top<D>>
    where
        D: Doom;

    /// Transforms `self` into a [`Result<T, Stack>`]: [`Some(value)`] is mapped to [`Ok(value)`],
    /// [`None`] is mapped to a [`Stack`] wrapping `doom`.
    ///
    /// _(See [`Doom::into_stack`] for additional information.)_
    ///
    /// [`Ok(value)`]: Ok
    /// [`Some(value)`]: Some
    fn or_stack<D>(self, doom: D) -> Result<T, Stack>
    where
        D: Doom;

    /// Syntax sugar for [`OptionExt::or_top`], then [`ResultExt::spot`].
    ///
    /// Calling `option.or_tot(doom, location)` is equivalent to calling
    /// `option.or_top(doom).spot(location)`.
    fn or_tot<D>(self, doom: D, location: Location) -> Result<T, Top<D>>
    where
        D: Doom;

    /// Syntax sugar for [`OptionExt::or_stack`], then [`ResultExt::spot`].
    ///
    /// Calling `option.or_stot(doom, location)` is equivalent to calling
    /// `option.or_stack(doom).spot(location)`.
    fn or_stot<D>(self, doom: D, location: Location) -> Result<T, Stack>
    where
        D: Doom;

    /// Returns the [`Option`]'s [`Some`] value or hangs indefinitely.
    ///
    /// Note: [`OptionExt::unwrap_or_hang`] is silent. If you need reporting on stderr, consider
    /// using [`OptionExt::expect_or_hang`].
    fn unwrap_or_hang(self) -> impl Future<Output = T> + Send
    where
        T: Send;

    /// Returns the [`Option`]'s [`Some`] value or hangs indefinitely after reporting `message` on
    /// stderr.
    fn expect_or_hang(self, message: &str) -> impl Future<Output = T> + Send
    where
        T: Send;
}

impl<T> OptionExt<T> for Option<T> {
    fn or_top<D>(self, doom: D) -> Result<T, Top<D>>
    where
        D: Doom,
    {
        self.ok_or_else(|| doom.into_top())
    }

    fn or_stack<D>(self, doom: D) -> Result<T, Stack>
    where
        D: Doom,
    {
        self.ok_or_else(|| doom.into_stack())
    }

    fn or_tot<D>(self, doom: D, location: Location) -> Result<T, Top<D>>
    where
        D: Doom,
    {
        self.or_top(doom).spot(location)
    }

    fn or_stot<D>(self, doom: D, location: Location) -> Result<T, Stack>
    where
        D: Doom,
    {
        self.or_stack(doom).spot(location)
    }

    async fn unwrap_or_hang(self) -> T
    where
        T: Send,
    {
        match self {
            Some(value) => value,
            None => future::pending().await,
        }
    }

    async fn expect_or_hang(self, message: &str) -> T
    where
        T: Send,
    {
        match self {
            Some(value) => value,
            None => {
                eprintln!("{message}");
                future::pending().await
            }
        }
    }
}
