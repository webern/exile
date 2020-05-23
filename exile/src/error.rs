// Copyright 2020 by Matthew James Briggs

#![allow(clippy::default_trait_access)]

use snafu::{Backtrace, Snafu};

use crate::parser::Position;

/// Alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ParseLocation {
    pub line: u64,
    pub column: u64,
}

/// The error type for this library.
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    /// A failure while parsing xml.
    #[snafu(display("Failure while parsing: {:?}", position))]
    Parse {
        position: Position,
        backtrace: Backtrace,
    },
    IoRead {
        parse_location: ParseLocation,
        source: std::io::Error,
        backtrace: Backtrace,
    },
    #[snafu(display("Oh no! A bug in the program: '{}'", message))]
    Bug { message: String },
}

// used in `std::io::Read` implementations
impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self {
        Self::new(std::io::ErrorKind::Other, err)
    }
}
