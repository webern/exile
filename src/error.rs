/*!
The public error type for this library.
!*/

#![allow(clippy::default_trait_access)]

use core::fmt;
use std::fmt::{Display, Formatter};

use crate::xdoc::error::XDocErr;
use crate::ParseError;

/// Alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for this library.
#[derive(Debug)]
pub enum Error {
    /// A syntax error encountered when parsing an XML document.
    Parse(ParseError),
    /// An error related to the `Document` model.
    XdocErr(XDocErr),
    /// Any other error.
    Other(OtherError),
}

impl Display for crate::error::Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(pe) => pe.fmt(f),
            Error::XdocErr(xe) => xe.fmt(f),
            Error::Other(oe) => oe.fmt(f),
        }
    }
}

impl std::error::Error for crate::error::Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(e) => convert_err(&e.source),
            Error::XdocErr(e) => convert_err(&e.source),
            Error::Other(e) => convert_err(&e.source),
        }
    }
}

impl From<XDocErr> for Error {
    fn from(xe: XDocErr) -> Self {
        Error::XdocErr(xe)
    }
}

fn convert_err<'a>(
    e: &'a Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
) -> Option<&'a (dyn std::error::Error + 'static)> {
    e.as_ref().map(|e| e.as_ref() as _)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// public error data
////////////////////////////////////////////////////////////////////////////////////////////////////

/// The Rust sourcecode file and line number which is the 'throw' site of an error.
#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct ThrowSite {
    /// The rust source file where the error was thrown, i.e. file!()
    pub file: String,
    /// The rust source line number where the error was thrown, i.e. line!()
    pub line: u32,
}

impl Display for ThrowSite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file, self.line)
    }
}

/// Represents any error that is not related to the syntax of the XML file.
#[derive(Debug, Default)]
pub struct OtherError {
    /// The location in this library's sourcecode where the error was thrown.
    pub throw_site: ThrowSite,
    /// An optional error message.
    pub message: Option<String>,
    /// An optional underlying error that is being wrapped.
    pub source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl Display for OtherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.throw_site.fmt(f)?;
        if let Some(msg) = &self.message {
            if !msg.is_empty() {
                write!(f, " - {}", msg)?;
            }
        }
        if let Some(e) = &self.source {
            write!(f, " - caused by: ")?;
            e.fmt(f)?;
        }
        Ok(())
    }
}

impl From<ParseError> for Error {
    fn from(pe: ParseError) -> Self {
        Self::Parse(pe)
    }
}
