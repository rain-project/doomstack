use crate::{Description, Stack, Top};
use std::error;

pub trait Doom: error::Error + 'static + Sized + Send + Sync {
    const VARIANTS: &'static [&'static str];

    fn acquire();
    fn release();
    fn store() -> bool;

    fn variant(&self) -> usize;
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
