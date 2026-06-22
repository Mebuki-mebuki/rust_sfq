#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SourceLocation {
    pub(crate) file: &'static str,
    pub(crate) line: u32,
    pub(crate) column: u32,
}

impl SourceLocation {
    #[track_caller]
    pub(crate) fn caller() -> Self {
        let location = std::panic::Location::caller();
        Self {
            file: location.file(),
            line: location.line(),
            column: location.column(),
        }
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Located<T> {
    pub(crate) value: T,
    pub(crate) location: SourceLocation,
}

impl<T> Located<T> {
    pub(crate) fn new(value: T, location: SourceLocation) -> Self {
        Self { value, location }
    }
}
