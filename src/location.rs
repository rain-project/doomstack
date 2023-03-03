#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub file: &'static str,
    pub line: u32,
}
