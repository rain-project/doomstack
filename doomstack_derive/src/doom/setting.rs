use crate::doom::{Description, KeepOriginal, Wrap};

pub(crate) enum Setting {
    Description(Description),
    KeepOriginal(KeepOriginal),
    Wrap(Wrap),
}
