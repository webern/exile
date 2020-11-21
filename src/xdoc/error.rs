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
    pub source: Option<Box<dyn Error>>,
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

macro_rules! wrap {
    // Base case:
    ($err:expr) => (Err($crate::xdoc::error::XDocErr {
        message: "an error occurred".to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
    ($err:expr, $msg:expr) => (Err($crate::xdoc::error::XDocErr {
        message: $msg.to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
    ($err:expr, $fmt:expr, $($arg:expr),+) => (Err($crate::xdoc::error::XDocErr {
        message: format!($fmt, $($arg),+),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
}

macro_rules! better_wrap {
    ($result:expr) => {
        match $result {
            Ok(value) => Ok(value),
            Err(e) => wrap!(e),
        }
    };
}

// a convenience macro for creating a Result::Err
macro_rules! raise {
    // Base case:
    ($msg:expr) => (Err($crate::xdoc::error::XDocErr {
        message: $msg.to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: None,
    }));
    ($fmt:expr, $($arg:expr),+) => ($Err($crate::xdoc::error::XErr {
        message: format!($fmt, $($arg),+),
        file: file!().to_string(),
        line: line!() as u64,
        source: None,
    }));
}
