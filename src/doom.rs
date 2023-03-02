use crate::{Description, Stack, Top};

/// [`Doom`] is a trait representing the basic expectations for [`doomstack`] errors.
///
/// Errors must describe themselves via [`Doom::tag`] (which should identify the error
/// type with a short, one-word, statically-defined tag) and [`Doom::description`]
/// (which should provide a one-sentence description of the error, and can be either
/// statically or dinamically defined).
///
/// [`doomstack`]: crate
/// [`Doom`]: crate::Doom
pub trait Doom: 'static + Sized + Send + Sync {
    fn keep_original() -> bool;

    fn tag(&self) -> &'static str;
    fn description(&self) -> Description;

    fn into_top(self) -> Top<Self> {
        Stack::new().push(self)
    }

    fn into_stack(self) -> Stack {
        self.into_top().into()
    }

    fn fail<O>(self) -> Result<O, Top<Self>> {
        Err(self.into_top())
    }

    fn fail_as_stack<O>(self) -> Result<O, Stack> {
        Err(self.into_stack())
    }
}
