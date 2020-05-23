// Copyright 2020 by Matthew James Briggs

#![allow(clippy::default_trait_access)]

use core::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

use crate::parser::{ParserState, Position};

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

impl From<Position> for XMLSite {
    fn from(p: Position) -> Self {
        XMLSite::from_position(&p)
    }
}

impl From<XMLSite> for Position {
    fn from(x: XMLSite) -> Self {
        x.to_position()
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// PRIVATE

impl XMLSite {
    fn from_position(p: &Position) -> Self {
        Self {
            line: p.line,
            column: p.column,
            position: p.absolute,
        }
    }

    fn to_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
            absolute: self.position,
        }
    }
}

#[macro_export]
macro_rules! throw_site {
    () => {
        ThrowSite {
            file: file!().to_owned(),
            line: line!(),
        }
    };
}

fn box_err<E>(err: Option<E>) -> Option<Box<dyn std::error::Error>>
where
    E: std::error::Error + 'static,
{
    match err {
        None => None,
        Some(e) => Some(e.into()),
    }
}

fn parse_err<S, E>(
    parser_state: &ParserState,
    throw_site: ThrowSite,
    message: Option<S>,
    source: Option<E>,
) -> crate::error::Error
where
    S: Into<String>,
    E: std::error::Error + 'static,
{
    crate::error::Error::Parse(ParseError {
        throw_site,
        xml_site: XMLSite::from_position(&parser_state.position),
        message: match message {
            None => None,
            Some(s) => Some(s.into()),
        },
        source: box_err(source),
    })
}

fn parse_result<S, E, T>(
    parser_state: &ParserState,
    throw_site: ThrowSite,
    message: Option<S>,
    source: Option<E>,
) -> crate::error::Result<T>
where
    S: Into<String>,
    E: std::error::Error + 'static,
{
    Err(parse_err(parser_state, throw_site, message, source))
}

/// Creates a ParseError object.
/// parser_state: required as the first object
/// message: optional, can be a string or a format
#[macro_export]
macro_rules! parse_err {
    // required: first argument must be the ParserState object
    ($parser_state:expr) => {
        parse_err(
            $parser_state,
            throw_site!(),
            Option::<String>::None,
            Option::<crate::error::Error>::None,
        )
    };
    // optional: second argument can be a simple string message
    ($parser_state:expr, $msg:expr) => {
        parse_err(
            $parser_state,
            throw_site!(),
            Some($msg),
            Option::<crate::error::Error>::None,
        )
    };
    ($parser_state:expr, $fmt:expr, $($arg:expr),+) => {
        parse_err(
            $parser_state,
            throw_site!(),
            Some(format!($fmt, $($arg),+)),
            Option::<crate::error::Error>::None,
        )
    };
}

/// Creates a Result populated by a ParseError
/// parser_state: required as the first object
/// message: optional, can be a string or a format
#[macro_export]
macro_rules! parse {
    // required: first argument must be the ParserState object
    ($parser_state:expr) => { Err(parse_err!($parser_state)) };
    // optional: second argument can be a simple string message
    ($parser_state:expr, $msg:expr) => { Err(parse_err!($parser_state, $msg) ) };
    // optional: format!
    ($parser_state:expr, $fmt:expr, $($arg:expr),+) => {
        Err(parse_err!($parser_state, $fmt, $($arg),+))
    };
}

#[test]
fn parse_err_test_simple() {
    let mut p = ParserState::default();
    p.position.line = 2;
    p.position.absolute = 31;
    p.position.column = 10;
    p.c = 'o';
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let e = parse_err!(&p);
    if let Error::Parse(pe) = e {
        assert_eq!(2, pe.xml_site.line);
        assert_eq!(31, pe.xml_site.position);
        assert_eq!(10, pe.xml_site.column);
        assert_eq!(expected_file, pe.throw_site.file);
        assert_eq!(expected_line, pe.throw_site.line);
        assert!(pe.message.is_none());
    } else {
        panic!("wrong error type");
    }
}

#[test]
fn parse_err_test_message() {
    let mut p = ParserState::default();
    p.position.line = 2;
    p.position.absolute = 31;
    p.position.column = 10;
    p.c = 'o';
    let message = "some message";
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let e = parse_err!(&p, message);
    if let Error::Parse(pe) = e {
        assert_eq!(2, pe.xml_site.line);
        assert_eq!(31, pe.xml_site.position);
        assert_eq!(10, pe.xml_site.column);
        assert_eq!(expected_file, pe.throw_site.file);
        assert_eq!(expected_line, pe.throw_site.line);
        assert_eq!(message, pe.message.unwrap());
    } else {
        panic!("wrong error type");
    }
}

#[test]
fn parse_err_test_message_fmt() {
    let mut p = ParserState::default();
    p.position.line = 5;
    p.position.absolute = 45;
    p.position.column = 9;
    p.c = 'o';
    let message = format!("some message {}", 6);
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let e = parse_err!(&p, "some message {}", 6);
    if let Error::Parse(pe) = e {
        assert_eq!(5, pe.xml_site.line);
        assert_eq!(45, pe.xml_site.position);
        assert_eq!(9, pe.xml_site.column);
        assert_eq!(expected_file, pe.throw_site.file);
        assert_eq!(expected_line, pe.throw_site.line);
        assert_eq!(message, pe.message.unwrap());
    } else {
        panic!("wrong error type");
    }
}

#[test]
fn parse_result_test_simple() {
    let mut p = ParserState::default();
    p.position.line = 2;
    p.position.absolute = 31;
    p.position.column = 10;
    p.c = 'o';
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let result: crate::error::Result<u32> = parse!(&p);
    let e = result.err().unwrap();
    if let Error::Parse(pe) = e {
        assert_eq!(2, pe.xml_site.line);
        assert_eq!(31, pe.xml_site.position);
        assert_eq!(10, pe.xml_site.column);
        assert_eq!(expected_file, pe.throw_site.file);
        assert_eq!(expected_line, pe.throw_site.line);
        assert!(pe.message.is_none());
    } else {
        panic!("wrong error type");
    }
}

#[test]
fn parse_result_test_message() {
    let mut p = ParserState::default();
    p.position.line = 2;
    p.position.absolute = 31;
    p.position.column = 10;
    p.c = 'o';
    let message = "some message";
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let result: Result<Option<String>> = parse!(&p, message);
    let e = result.err().unwrap();
    if let Error::Parse(pe) = e {
        assert_eq!(2, pe.xml_site.line);
        assert_eq!(31, pe.xml_site.position);
        assert_eq!(10, pe.xml_site.column);
        assert_eq!(expected_file, pe.throw_site.file);
        assert_eq!(expected_line, pe.throw_site.line);
        assert_eq!(message, pe.message.unwrap());
    } else {
        panic!("wrong error type");
    }
}

#[test]
fn parse_result_test_message_fmt() {
    use xdoc::ElementData;
    let mut p = ParserState::default();
    p.position.line = 5;
    p.position.absolute = 45;
    p.position.column = 9;
    p.c = 'o';
    let message = format!("some message {}", 6);
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let result: Result<ElementData> = parse!(&p, "some message {}", 6);
    let e = result.err().unwrap();
    if let Error::Parse(pe) = e {
        assert_eq!(5, pe.xml_site.line);
        assert_eq!(45, pe.xml_site.position);
        assert_eq!(9, pe.xml_site.column);
        assert_eq!(expected_file, pe.throw_site.file);
        assert_eq!(expected_line, pe.throw_site.line);
        assert_eq!(message, pe.message.unwrap());
    } else {
        panic!("wrong error type");
    }
}
