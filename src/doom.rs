use crate::{Description, Location, ResultExt, Stack, Top};

/// [`Doom`] is a trait representing the basic expectations for [doomstack](crate) errors.
///
/// Errors must describe themselves via [`Doom::tag`] (which should identify the error
/// type with a short, one-word, statically-defined tag, usually the type or variant of
/// the error) and [`Doom::description`] (which should provide a one-sentence description
/// of the error, and can be either statically or dynamically defined, see [`Description`]).
///
/// Optionally, a [doomstack](crate) error can override [`Doom::keep_original`] to indicate
/// whether or not the original error should be kept (in a [`Box<dyn Any>`], see
/// [`crate`]-level documentation) when the error is archived in an [`Entry`] (a default
/// implementation of [`Doom::keep_original`] is provided, which always returns `false`).
///
/// # Derivable
///
/// [`Doom`] can be derived for both structs and enums.
///
/// #### The `description` attribute
///
/// To derive [`Doom`], all you need to do is provide a `#[doom(description(...))]`
/// attribute for every group of fields in your type. This means: one description
/// per struct; one description per enum variant.
///
/// ```
/// # use doomstack::Doom;
/// #
/// #[derive(Doom)]
/// #[doom(description("Hose malfunctioned (pressure was {pressure}, {holes} holes formed)"))]
/// struct HoseError {
///     pressure: f32,
///     holes: u32,
/// }
///
/// #[derive(Doom)]
/// enum IrrigationError {
///     #[doom(description("Faucet broken."))]
///     FaucetBroken,
///     #[doom(description("Forgot to water for {days} days."))]
///     ForgotToWater { days: u32 },
/// }
/// ```
///
/// The `#[doom(description(...))]` attribute can be used very much like the [`format!`]
/// macro: one format string literal, optionally followed by arguments to format. If you
/// want to include the fields of your struct or variant in your description, keep the
/// following in mind:
///
///  - The `self` keyword is always at your disposal, capturing an immutable reference
///    to your type (struct or enum).
///  - If you are describing a group of named fields, every field is available to you
///    by its name, captured as an immutable reference.
///  - If you are describing a group of unnamed fields, every field is available to
///    you by its index, prefixed by an underscore (`_`), captured as an immutable
///    reference.
///
/// This means all of the following are allowed:
/// ```
/// # use doomstack::Doom;
/// #
/// #[derive(Doom)]
/// #[doom(description("Error code: {}, severity: {}, message: {_1}", self.0, self.severity()))]
/// struct TupleLikeError(u32, String);
///
/// impl TupleLikeError {
///     fn severity(&self) -> u32 {
///         // ...
///         # unimplemented!()
///     }
/// }
///
/// #[derive(Doom)]
/// enum EnumError {
///     #[doom(description("Error code: {code}, message: {message}"))]
///     NamedFields {
///         code: u32,
///         message: String
///     },
///     #[doom(description("Error code: {_0}, message: {_1}"))]
///     UnnamedFields(u32, String),
///     #[doom(description("Error code: {}", match self { EnumError::UnnecessarilyContrivedExample(code) => code, _ => unreachable!()}))]
///     UnnecessarilyContrivedExample(u32),
/// }
/// ```
///
/// # Manual implementation
///
/// ```
/// use doomstack::{Description, Doom};
///
/// enum GardeningError {
///     TooMuchWater,
///     ForgotFertilizer,
///     DroppedFlowerpot { height: f64 },
/// }
///
/// impl Doom for GardeningError {
///     fn tag(&self) -> &'static str {
///         match self {
///             GardeningError::TooMuchWater => "GardeningError::TooMuchWater",
///             GardeningError::ForgotFertilizer => "GardeningError::ForgotFertilizer",
///             GardeningError::DroppedFlowerpot { .. } => "GardeningError::DroppedFlowerpot",
///         }
///     }
///
///     fn description(&self) -> Description {
///         match self {
///             GardeningError::TooMuchWater => {
///                 Description::Static("Added too much water, plants drowned")
///             }
///             GardeningError::ForgotFertilizer => {
///                 Description::Static("Forgot to add fertilizer, plants starved")
///             }
///             GardeningError::DroppedFlowerpot { height } => Description::Owned(format!(
///                 "Dropped flowerpot from {height} meters, plants crashed and many injured"
///             )),
///         }
///     }
/// }
/// ```
///
/// [`doomstack`]: crate
/// [`Entry`]: crate::Entry
/// [`Box<dyn Any>`]: std::any::Any
pub trait Doom: 'static + Sized + Send + Sync {
    /// A short, one-word, statically defined tag, used to identify the error type.
    fn tag(&self) -> &'static str;

    /// A one-sentence description of the error. It can be either static or dynamic.
    fn description(&self) -> Description;

    /// Indicates whether or the original error should be kept (in a [`Box<dyn Any>`]) when
    /// the error is archived in an [`Entry`]. A default implementation of [`Doom::keep_original`]
    /// is provided, which always returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use doomstack::{Description, Doom, Entry};
    ///
    /// struct Oupsie(u64);
    ///
    /// impl Doom for Oupsie {
    ///     fn tag(&self) -> &'static str {
    ///         "Oupsie"
    ///     }
    ///
    ///     fn description(&self) -> Description {
    ///         Description::Static("Made a mess")
    ///     }
    ///
    ///     fn keep_original(&self) -> bool {
    ///         true
    ///     }
    /// }
    ///
    /// let oupsie = Oupsie(42);
    /// let entry = Entry::archive(oupsie);
    ///
    /// let value = entry
    ///     .original()
    ///     .unwrap()
    ///     .downcast_ref::<Oupsie>()
    ///     .unwrap();
    ///
    /// assert_eq!(value.0, 42);
    /// ```
    ///
    /// [`Entry`]: crate::Entry
    /// [`Box<dyn Any>`]: std::any::Any
    fn keep_original(&self) -> bool {
        false
    }

    /// Wraps `self` into a [`Top<Self>`] whose [`doom()`] is `self` and whose [`stack()`] has no entries.
    ///
    /// # Examples
    ///
    /// ```
    /// use doomstack::{Description, Doom};
    ///
    /// #[derive(Debug, Clone, PartialEq)]
    /// struct Oupsie(u64);
    ///
    /// impl Doom for Oupsie {
    ///     fn tag(&self) -> &'static str {
    ///         "Oupsie"
    ///     }
    ///
    ///     fn description(&self) -> Description {
    ///         Description::Static("Made a mess")
    ///     }
    /// }
    ///
    /// let oupsie = Oupsie(42);
    /// let top = oupsie.clone().into_top();
    ///
    /// assert_eq!(top.doom(), &oupsie);
    /// ```
    ///
    /// [`doom()`]: crate::Top::doom
    /// [`stack()`]: crate::Top::stack
    fn into_top(self) -> Top<Self> {
        Stack::new().push(self)
    }

    /// Wraps `self` into a [`Stack`] whose only [`Entry`] archives `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use doomstack::{Description, Doom};
    ///
    /// struct Oupsie;
    ///
    /// impl Doom for Oupsie {
    ///     fn tag(&self) -> &'static str {
    ///         "Oupsie"
    ///     }
    ///
    ///     fn description(&self) -> Description {
    ///         Description::Static("Made a mess")
    ///     }
    /// }
    ///
    /// let stack = Oupsie.into_stack();
    ///
    /// assert_eq!(stack.entries().next().unwrap().tag(), "Oupsie");
    /// ```
    ///
    /// [`Entry`]: crate::Entry
    fn into_stack(self) -> Stack {
        self.into_top().into()
    }

    /// Wraps `self` into a [`Top<Self>`] (as in [`Doom::into_top()`]), then into the `Err`
    /// variant of a [`Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// use doomstack::{Description, Doom, Top};
    ///
    /// struct NotEven;
    ///
    /// impl Doom for NotEven {
    ///     fn tag(&self) -> &'static str {
    ///         "NotEven"
    ///     }
    ///
    ///     fn description(&self) -> Description {
    ///         Description::Static("The number provided is not even")
    ///     }
    /// }
    ///
    /// fn checked_half(n: u32) -> Result<u32, Top<NotEven>> {
    ///     if n % 2 == 0 {
    ///         Ok(n / 2)
    ///     } else {
    ///         NotEven.fail()
    ///     }
    /// }
    /// ```
    ///
    fn fail<O>(self) -> Result<O, Top<Self>> {
        Err(self.into_top())
    }

    /// Syntax sugar for [`Doom::fail`], then [`ResultExt::spot`].
    ///
    /// Calling `doom.fail().spot(location)` is equivalent to calling `doom.fot(location)`.
    fn fot<O>(self, location: Location) -> Result<O, Top<Self>> {
        self.fail().spot(location)
    }

    /// Wraps `self` into a `Stack` (as in [`Doom::into_stack()`]), then into the `Err`
    /// variant of a [`Result`].
    ///
    /// # Examples
    ///
    /// ```
    /// use doomstack::{Description, Doom, Stack};
    ///
    /// struct NotEven;
    ///
    /// impl Doom for NotEven {
    ///     fn tag(&self) -> &'static str {
    ///         "NotEven"
    ///     }
    ///
    ///     fn description(&self) -> Description {
    ///         Description::Static("The number provided is not even")
    ///     }
    /// }
    ///
    /// fn checked_half(n: u32) -> Result<u32, Stack> {
    ///     if n % 2 == 0 {
    ///         Ok(n / 2)
    ///     } else {
    ///         NotEven.fail_as_stack()
    ///     }
    /// }
    /// ```
    ///
    fn fail_as_stack<O>(self) -> Result<O, Stack> {
        Err(self.into_stack())
    }

    /// Syntax sugar for [`Doom::fail_as_stack`], then [`ResultExt::spot`].
    ///
    /// Calling `doom.fail_as_stack().spot(location)` is equivalent to calling `doom.fot_as_stack(location)`.
    fn fot_as_stack<O>(self, location: Location) -> Result<O, Stack> {
        self.fail_as_stack().spot(location)
    }
}
