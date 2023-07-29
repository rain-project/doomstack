use crate::doom::{Description, Wrap};

pub(crate) enum Setting {
    Description(Description),
    Wrap(Wrap),
}
