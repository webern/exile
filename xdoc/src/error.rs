use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, XErr>;

#[derive(Debug)]
pub struct XErr {
    pub message: String,
    pub file: String,
    pub line: u64,
    pub source: Option<Box<dyn Error>>,
}

impl fmt::Display for XErr {
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

impl Error for XErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(src) = &self.source {
            return Some(src.as_ref());
        }
        None
    }
}

#[macro_export]
macro_rules! wrap {
    // Base case:
    ($err:expr) => (Err($crate::error::XErr {
        message: "an error occurred".to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
    ($err:expr, $msg:expr) => (Err($crate::error::XErr {
        message: $msg.to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
    ($err:expr, $fmt:expr, $($arg:expr),+) => (Err($crate::error::XErr {
        message: format!($fmt, $($arg),+),
        file: file!().to_string(),
        line: line!() as u64,
        source: Some($err.into()),
    }));
}

#[macro_export]
macro_rules! better_wrap {
    ($result:expr) => {
        match $result {
            Ok(value) => Ok(value),
            Err(e) => wrap!(e),
        }
    };
}

#[macro_export]
macro_rules! raise {
    // Base case:
    ($msg:expr) => (Err($crate::error::XErr {
        message: $msg.to_string(),
        file: file!().to_string(),
        line: line!() as u64,
        source: None,
    }));
    ($fmt:expr, $($arg:expr),+) => ($Err($crate::error::XErr {
        message: format!($fmt, $($arg),+),
        file: file!().to_string(),
        line: line!() as u64,
        source: None,
    }));
}
