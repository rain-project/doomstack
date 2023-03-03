use crate::{Description, Doom};
use std::{
    any::Any,
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};

/// An [`Entry`] is an element of an error [`Stack`], archiving a [`Doom`] error.
///
/// In archiving a [`Doom`] error, an [`Entry`] captures the error's [`Doom::description()`]
/// and [`Doom::tag()`], along with a copy of the original error (held in a [`Box<dyn Any>`]),
/// if prescribed by [`Doom::keep_original()`].
///
/// [`Stack`]: crate::Stack
#[derive(Clone)]
pub struct Entry {
    tag: &'static str,
    description: Description,
    location: Option<(&'static str, u32)>,
    original: Option<Arc<dyn Any + Send + Sync>>,
}

impl Entry {
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

    pub fn tag(&self) -> &'static str {
        self.tag
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn location(&self) -> Option<(&'static str, u32)> {
        self.location
    }

    pub fn original(&self) -> Option<&(dyn Any + Send + Sync)> {
        self.original.as_ref().map(AsRef::as_ref)
    }

    pub fn spot(&mut self, location: (&'static str, u32)) {
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
            write!(
                f,
                "[{} @ {}:{}] {}",
                self.tag, location.0, location.1, self.description
            )?;
        } else {
            write!(f, "[{}] {}", self.tag, self.description)?;
        }

        Ok(())
    }
}
