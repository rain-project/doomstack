use crate::{Description, Stack, Top};
use std::error;

pub trait Doom: error::Error + 'static + Sized + Send + Sync {
    fn store() -> bool;

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
