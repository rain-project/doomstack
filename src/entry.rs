use crate::{Description, Doom, Location};
use std::{
    any::Any,
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};

/// An [`Entry`] is an element of an error [`Stack`], archiving a [`Doom`] error.
///
/// In archiving a [`Doom`] error, an [`Entry`] captures the error's [`Doom::description()`]
/// and [`Doom::tag()`], along with a copy of the original error (held in a [`Box<dyn Any>`],
/// only if prescribed by [`Doom::keep_original()`]), and the (optional) [`Location`] at
/// which the [`Entry`] was last [`spot()`]-ted.
///
/// # Examples
///
/// ```
/// use doomstack::{Description, Doom, Entry, Location, here};
///
/// struct Oupsie {
///     details: String
/// }
///
/// impl Doom for Oupsie {
///     fn tag(&self) -> &'static str {
///         "Oupsie"
///     }
///
///     fn description(&self) -> Description {
///         Description::Owned(format!("Made a mess: {}", self.details))
///     }
///
///     fn keep_original(&self) -> bool {
///         true
///     }
/// }
///
/// let oupsie = Oupsie { details: "got distracted".to_string() };
/// let mut entry = Entry::archive(oupsie);
///
/// assert_eq!(entry.tag(), "Oupsie");
/// assert_eq!(entry.description(), "Made a mess: got distracted");
/// assert_eq!(entry.location(), None);
///
/// entry.spot(here!());
///
/// assert_eq!(
///     entry.location(),
///     Some(Location {
///         file: file!(),
///         line: line!() - 6 // `spot!()` happened six lines ago
///     })
/// );
///
/// let original = entry.original().unwrap();
/// let original = original.downcast_ref::<Oupsie>().unwrap();
///
/// assert_eq!(&original.details, "got distracted");
/// ```
///
/// [`Stack`]: crate::Stack
/// [`spot()`]: crate::Entry::spot
#[derive(Clone)]
pub struct Entry {
    tag: &'static str,
    description: Description,
    location: Option<Location>,
    original: Option<Arc<dyn Any + Send + Sync>>,
}

impl Entry {
    /// Archives a [`Doom`] error, capturing its [`Doom::tag()`], [`Doom::description()`].
    ///
    /// If prescribed by [`Doom::keep_original()`]), [`Entry::archive`] additionally stores
    /// the original error in a [`Box<dyn Any>`] for later retrieval.
    ///
    /// [`Box<dyn Any>`]: Any
    pub fn archive<D>(doom: D) -> Self
    where
        D: Doom,
    {
        let mut entry = Entry {
            tag: doom.tag(),
            description: Doom::description(&doom),
            location: None,
            original: None,
        };

        if doom.keep_original() {
            entry.original = Some(Arc::new(doom));
        }

        entry
    }

    /// Returns the [`Doom::tag()`] the [`Entry`] captured upon archiving a [`Doom`] error.
    pub fn tag(&self) -> &'static str {
        self.tag
    }

    /// Returns the `[Doom::description()]` the [`Entry`] captured upon archiving a [`Doom`] error.
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the (optional) [`Location`] at which `self` was last [`spot()`]-ted.
    ///
    /// [`spot()`]: crate::Entry::spot
    pub fn location(&self) -> Option<Location> {
        self.location
    }

    /// Returns, if available, the original [`Doom`] error stored upon archival.
    pub fn original(&self) -> Option<&(dyn Any + Send + Sync)> {
        self.original.as_ref().map(AsRef::as_ref)
    }

    /// Sets the [`Location`] the [`Entry`] was last spotted at.
    ///
    /// Usually used in conjuction with the [`here!()`] macro, the [`Entry::spot`]
    /// updates the [`Location`] (filename and line number) at which the [`Entry`]
    /// was last spotted. [`Entry`]-ies are usually [`spot()`]-ed where they are
    /// created and pushed on a [`Stack`] or [`Top`]. Depending on your needs,
    /// you may decide to [`spot()`] an [`Entry`] elsewhere, as it propagates
    /// through your code. Remember, however, that [`Entry::spot`] _updates_ the
    /// [`Entry`]'s last spotted [`Location`], overwriting whatever [`Location`]
    /// the [`Entry`] was previously [`spot()`]-ed at. To track an error as it
    /// propagates through your code, consider [`push()`]-ing multiple [`Entry`]-ies
    /// errors onto a [`Stack`] or [`Top`], tagging each [`Entry`] with an
    /// appropriate [`Location`] (see [`crate`]-level documentation for additional
    /// detail).
    ///
    /// For everyday operations, it is unlikely you'll need to call [`Entry::spot`]
    /// directly. More often than not, you will rather call [`ResultExt::spot`],
    /// [`Top::spot`] or [`Stack::spot`], all of which forward your call to the
    /// appropriate [`Top`] or [`Entry`].
    ///
    /// [`here!()`]: crate::here!
    /// [`spot()`]: Entry::spot
    /// [`push()`]: crate::Top::push
    /// [`ResultExt::spot`]: crate::ResultExt::spot
    /// [`Stack`]: crate::Stack
    /// [`Stack::spot`]: crate::Stack::spot
    /// [`Top`]: crate::Top
    /// [`Top::spot`]: crate::Top::spot
    pub fn spot(&mut self, location: Location) {
        self.location = Some(location);
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.tag)
    }
}

impl Debug for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(location) = self.location {
            write!(f, "[{} @ {}] {}", self.tag, location, self.description)?;
        } else {
            write!(f, "[{}] {}", self.tag, self.description)?;
        }

        Ok(())
    }
}
