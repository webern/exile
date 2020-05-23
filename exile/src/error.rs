// Copyright 2020 by Matthew James Briggs

#![allow(clippy::default_trait_access)]

use core::fmt;
use std::fmt::{Display, Formatter};

/// Alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct ParseLocation {
    pub line: u64,
    pub column: u64,
}

/// The error type for this library.
#[derive(Debug)]
pub enum Error {
    Parse(ParseError),
    IoRead {
        parse_location: ParseLocation,
        source: std::io::Error,
    },
    Bug {
        message: String,
    },
}

impl Display for crate::error::Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(pe) => pe.fmt(f),
            Error::IoRead { .. } => Ok(()),
            Error::Bug { .. } => Ok(()),
        }
    }
}

/// The sourcecode Rust file and line number which is the 'throw' site of an error.
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

/// The position in the XML file that the parser was at when the error was thrown. Ideally this
/// will match the exact location where an XML file first violates XML syntax. These numbers are
/// 1-based, i.e. line 1 is the first first line of the file, column 1 is the leftmost character of
/// a line, and position 1 is the first character of the document.
#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct XMLSite {
    /// The line in the XML file where an error was encountered. 1-based indexing.
    pub line: u64,
    /// The character within the line where an error was encountered. 1-based indexing.
    pub column: u64,
    /// The absolute character position within the line where an error was encountered. 1-based.
    pub position: u64,
}

impl From<crate::parser::Position> for XMLSite {
    fn from(p: crate::parser::Position) -> Self {
        XMLSite {
            line: p.line,
            column: p.column,
            position: p.absolute,
        }
    }
}

impl From<XMLSite> for crate::parser::Position {
    fn from(x: XMLSite) -> Self {
        crate::parser::Position {
            line: x.line,
            column: x.column,
            absolute: x.position,
        }
    }
}

impl Display for XMLSite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "position: {}, line: {}, column: {}",
            self.position, self.line, self.column
        )
    }
}

#[derive(Debug, Default)]
pub struct ParseError {
    pub throw_site: ThrowSite,
    pub xml_site: XMLSite,
    pub message: Option<String>,
    pub source: Option<Box<dyn std::error::Error>>,
}

/// A cloned ParseError loses its source Error because the source Error cannot be cloned.
impl Clone for ParseError {
    fn clone(&self) -> Self {
        Self {
            throw_site: self.throw_site.clone(),
            xml_site: self.xml_site.clone(),
            message: self.message.clone(),
            source: None, // TODO - preserve the display of the source error
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.throw_site.fmt(f)?;
        write!(f, " xml ")?;
        self.xml_site.fmt(f)?;
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

impl std::error::Error for crate::error::Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(e) => {
                if let Some(s) = &e.source {
                    Some(s.as_ref())
                } else {
                    None
                }
            }
            Error::IoRead { .. } => None,
            Error::Bug { .. } => None,
        }
        // if let Some(src) = &self.source {
        //     return Some(src.as_ref());
        // }
        // None
    }
}

// used in `std::io::Read` implementations
// impl From<Error> for std::io::Error {
//     fn from(err: Error) -> Self {
//         Self::new(std::io::ErrorKind::Other, err)
//     }
// }
