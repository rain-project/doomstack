use crate::{Description, Stack, Top};

/// [`Doom`] is a trait representing the basic expectations for [`doomstack`] errors.
///
/// Errors must describe themselves via [`Doom::tag`] (which should identify the error
/// type with a short, one-word, statically-defined tag, usually the type or variant of 
/// the error) and [`Doom::description`] (which should provide a one-sentence description 
/// of the error, and can be either statically or dynamically defined).
///
/// Optionally, a [`doomstack`] error can override [`Doom::keep_original`] to indicate
/// whether or not the original error should be kept (in a [`Box<dyn Any>`], see
/// [`crate`]-level documentation) when the error is archived in an [`Entry`] (a default
/// implementation of [`Doom::keep_original`] is provided, which always returns `false`).
///
/// # Examples
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
    /// use doomstack::{Description, Doom};
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
    ///     fn keep_original() -> bool {
    ///         true
    ///     }
    /// }
    ///
    /// let oupsie = Oupsie(42);
    /// let stack = oupsie.into_stack();
    ///
    /// let value = stack.entries()[0]
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
    fn keep_original() -> bool {
        false
    }

    /// Wraps `self` into a [`Top<Self>`] whose [`top()`] is `self` and whose [`base()`] has no entries.
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
    /// assert_eq!(top.top(), &oupsie);
    /// ```
    ///
    /// [`top()`]: crate::Top::top
    /// [`base()`]: crate::Top::base
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
    /// assert_eq!(stack.entries()[0].tag(), "Oupsie");
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
}
