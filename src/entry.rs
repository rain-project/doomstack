use crate::{Description, Doom};
use std::{
    any::Any,
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
};

#[derive(Clone)]
pub struct Entry {
    tag: &'static str,
    description: Description,
    location: Option<(&'static str, u32)>,
    original: Option<Arc<dyn Any + Send + Sync>>,
}

impl Entry {
    pub(crate) fn new<D>(doom: &D) -> Self
    where
        D: Doom,
    {
        Entry {
            tag: doom.tag(),
            description: Doom::description(doom),
            location: None,
            original: None,
        }
    }

    pub(crate) fn store<D>(mut self, doom: D) -> Self
    where
        D: Doom,
    {
        self.original = Some(Arc::new(doom));
        self
    }

    pub fn spot(mut self, location: (&'static str, u32)) -> Self {
        self.location = Some(location);
        self
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

    pub fn original(&self) -> Option<&dyn Any> {
        match &self.original {
            Some(original) => Some(original.as_ref()),
            None => None,
        }
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
