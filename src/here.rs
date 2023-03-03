/// Expands to a [`Location`] containing the [`file`] and [`line`] in which it was invoked.
///
/// Lines are 1-based, so the first line in each file evaluates to 1, the second to 2, etc.
/// [`here!()`] can be used to quickly mark where a [`Stack`] or [`Top`] was `spot()`-ted.
///
/// # Examples
///
/// ```
/// use doomstack::here;
///
/// let location1 = here!();
/// let location2 = here!();
///
/// assert_eq!(location1.file, location2.file);
/// assert_eq!(location1.line + 1, location2.line);
/// ```
///
/// [`Location`]: crate::Location
/// [`file`]: crate::Location::file
/// [`line`]: crate::Location::line
/// [`here!()`]: crate::here!
/// [`Stack`]: crate::Stack
/// [`Top`]: crate::Top
#[macro_export]
macro_rules! here {
    () => {
        doomstack::Location {
            file: file!(),
            line: line!(),
        }
    };
}
