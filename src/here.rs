/// Expands to the file name and line in which it was invoked.
///
/// The expanded expression has type `(&'static str, u32)`. Lines are 1-based, so the first
/// line in each file evaluates to 1, the second to 2, etc. `here!()` can be used to
/// quickly mark where a `Stack` or `Top` was `spot()`-ted.
#[macro_export]
macro_rules! here {
    () => {
        (file!(), line!())
    };
}
