use std::error::Error;
use std::fmt;

/// The `Result` type for this library.
pub type Result<T> = std::result::Result<T, XDocErr>;

/// A generic error type for this library.
#[derive(Debug)]
pub struct XDocErr {
    /// The error message.
    pub message: String,
    /// The sourcecode file where the error was raised.
    pub file: String,
    /// The sourcecode line where the error was raised.
    pub line: u64,
    /// The underlying error that is being wrapped.
    pub source: Option<Box<dyn Error + Sync + Send + 'static>>,
}

impl fmt::Display for XDocErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(src) = &self.source {
            write!(
                f,
                "{}:{} {}: {}",
                self.file,
                self.line,
                self.message,
                src.as_ref()
            )
        } else {
            write!(f, "{}:{} {}", self.file, self.line, self.message)
        }
    }
}

impl Error for XDocErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(src) = &self.source {
            return Some(src.as_ref());
        }
        None
    }
}
