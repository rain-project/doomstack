use crate::{Doom, DoomResult, Location, Stack, Top};
use std::{
    fmt::Debug,
    future::{self, Future},
    process,
};

/// An interface extending the behavior of [`Result`] with [doomstack](crate) functionality.
///
/// # Example
///
/// ```
/// use doomstack::{prelude::*, Stack};
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
/// # use doomstack::prelude::*;
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
/// # use doomstack::prelude::*;
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
///
/// # Unwrapping
///
/// [`ResultExt`] offers two classes of utilities for unwrapping values beyond what [`Result`]
/// provides out of the box.
///
/// ## Unwrap or hang
///
/// In the asynchronous setting, you can use [`ResultExt::unwrap_or_hang`] /
/// [`ResultExt::expect_or_hang`] to get a [`Result`]'s [`Ok`] value or hang the task indefinitely.
/// Why would you ever want to do such a thing? For one thing, it greatly simplifies shutting down a
/// complex system of tasks!
///
/// ### Example
///
/// ```
/// use doomstack::prelude::*;
/// use tokio::sync::broadcast;
/// use tokio_util::sync::CancellationToken;
///
/// #[tokio::main]
/// async fn main() {
///     let (sender, mut receiver) = broadcast::channel(8);
///     let cancellation = CancellationToken::new();
///
///     {
///         let cancellation = cancellation.clone();
///
///         let task = async move {
///             loop {
///                 let message = receiver.recv().await.unwrap_or_hang().await;
///                 println!("{message}");
///             }
///         };
///
///         tokio::spawn(async move {
///             tokio::select! {
///                 _ = task => (),
///                 _ = cancellation.cancelled() => (),
///             }
///         });
///     }
///
///     sender.send("Hello there!");
///     sender.send("What a beautiful day!");
///
///     cancellation.cancel();
///     drop(sender);
/// }
/// ```
///
/// ### Discussion
///
/// In the example above, we spawn a `tokio` task that listens on a `broadcast` channel for
/// messages to print. The task is cancelled by a `CancellationToken`. Whenever we receive a
/// message, we simply [`ResultExt::unwrap_or_hang`] - no complex error handling required. On the
/// main task, we send a few messages, cancel the `CancellationToken`, then drop the `Sender`.
/// Simply due to scheduling, however, it is entirely possible that `Receiver::recv` will return
/// an error _before_ `CancellationToken::cancelled` returns. In this context, an  [`Err`] is just
/// a symptom that the task is about to be cancelled. The simplest course of action is to wait
/// around for that to happen. Without [`ResultExt::unwrap_or_hang`], we would have had to first
/// cancel the `CancellationToken`, wait on the task's `JoinHandle`, then drop the `Sender`.
///
/// ## Unwrap or exit
///
/// Sometimes an error is unrecoverable to the point that the whole process should be shut down as a
/// result. [`Result::unwrap`] on an [`Err`] value results in a panic, which by default unwinds only
/// the current thread, sometimes leaving your system limping in unexpected ways that might actually
/// be counterproductive if your goal is to effectively post-mortem what went wrong. In these cases,
/// you might want to reach out for [`ResultExt::unwrap_or_exit`] / [`ResultExt::expect_or_exit`].
/// When called on an [`Err`] value, they clearly report the fatality, then immediately terminate
/// the process with an non-zero exit code.
///
/// ### Intended semantics
///
/// An alternative to [`ResultExt::unwrap_or_exit`] is, of course, setting `panic = abort` in your
/// `Cargo.toml`, which has every panic result in process termination. [`ResultExt::unwrap_or_exit`]
/// makes termination an explicit and selective choice. Semantically, [`Result::unwrap`] is meant to
/// enforce at runtime an invariant that the developer expects the code to uphold. Conversely,
/// [`ResultExt::unwrap_or_exit`] indicates an error that might happen (e.g., as a result of the
/// program's environment - think a missing dependency, or failed access to a critical resource) but
/// cannot be recovered from without human intervention.
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

    /// Returns the [`Result`]'s [`Ok`] value or hangs indefinitely.
    ///
    /// Note: [`ResultExt::unwrap_or_hang`] is silent. If you need reporting on stderr, consider
    /// using [`ResultExt::expect_or_hang`].
    fn unwrap_or_hang(self) -> impl Future<Output = O> + Send
    where
        O: Send;

    /// Returns the [`Result`]'s [`Ok`] value or hangs indefinitely after reporting `message` and
    /// [`Err`] on stderr.
    fn expect_or_hang(self, message: &str) -> impl Future<Output = O> + Send
    where
        O: Send,
        E: Send + Debug;

    /// Returns the [`Result`]'s [`Ok`] value or terminates the process after reporting the [`Err`]
    /// as a fatality on stderr.
    fn unwrap_or_exit(self) -> O
    where
        E: Debug;

    /// Returns the [`Result`]'s [`Ok`] value or terminates the process after reporting `message`
    /// and [`Err`] as a fatality on stderr.
    fn expect_or_exit(self, message: &str) -> O
    where
        E: Debug;
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

    fn unwrap_or_hang(self) -> impl Future<Output = O> + Send
    where
        O: Send,
    {
        let value = self.ok();

        async move {
            match value {
                Some(value) => value,
                None => future::pending().await,
            }
        }
    }

    async fn expect_or_hang(self, message: &str) -> O
    where
        O: Send,
        E: Send + Debug,
    {
        match self {
            Ok(value) => value,

            Err(error) => {
                eprintln!("{message}:");
                eprintln!("{error:?}");
                future::pending().await
            }
        }
    }

    fn unwrap_or_exit(self) -> O
    where
        E: Debug,
    {
        match self {
            Ok(value) => value,

            Err(error) => {
                eprintln!("--------- Fatal error encountered ---------");
                eprintln!("{error:?}");
                eprintln!("------------ (killing process) ------------");
                process::exit(1);
            }
        }
    }

    fn expect_or_exit(self, message: &str) -> O
    where
        E: Debug,
    {
        match self {
            Ok(value) => value,

            Err(_) => {
                eprintln!("--------- Fatal error encountered ---------");
                eprintln!("{message:?}");
                eprintln!("------------ (killing process) ------------");
                process::exit(1);
            }
        }
    }
}
