// Copyright 2020 by Matthew James Briggs

#![allow(clippy::default_trait_access)]

use core::fmt;
use std::fmt::{Display, Formatter};

use crate::parser::ParserState;

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
    Other(OtherError),
}

impl Display for crate::error::Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(pe) => pe.fmt(f),
            Error::Other(oe) => oe.fmt(f),
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
    /// The character that was in scope with the error was encountered.
    pub character: char,
}

impl From<ParserState> for XMLSite {
    fn from(p: ParserState) -> Self {
        XMLSite::from_parser(&p)
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

#[derive(Debug, Default)]
pub struct OtherError {
    pub throw_site: ThrowSite,
    pub message: Option<String>,
    pub source: Option<Box<dyn std::error::Error>>,
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
            Error::Other(e) => {
                if let Some(s) = &e.source {
                    Some(s.as_ref())
                } else {
                    None
                }
            }
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
    fn from_parser(p: &ParserState) -> Self {
        Self {
            line: p.position.line,
            column: p.position.column,
            position: p.position.absolute,
            character: p.c,
        }
    }
}

#[macro_export]
macro_rules! throw_site {
    () => {
        crate::error::ThrowSite {
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

pub(crate) fn parse_err<S, E>(
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
        xml_site: XMLSite::from_parser(&parser_state),
        message: match message {
            None => None,
            Some(s) => Some(s.into()),
        },
        source: box_err(source),
    })
}

/// Creates a ParseError object.
/// parser_state: required as the first argument
/// message: optional, can be a string or a format
#[macro_export]
macro_rules! create_parser_error {
    // required: first argument must be the ParserState object
    ($parser_state:expr) => {
        crate::error::parse_err(
            $parser_state,
            throw_site!(),
            Option::<String>::None,
            Option::<crate::error::Error>::None,
        )
    };
    // optional: second argument can be a simple string message
    ($parser_state:expr, $msg:expr) => {
        crate::error::parse_err(
            $parser_state,
            throw_site!(),
            Some($msg),
            Option::<crate::error::Error>::None,
        )
    };
    ($parser_state:expr, $fmt:expr, $($arg:expr),+) => {
        crate::error::parse_err(
            $parser_state,
            throw_site!(),
            Some(format!($fmt, $($arg),+)),
            Option::<crate::error::Error>::None,
        )
    };
}

#[macro_export]
macro_rules! raise {
    () => {
        Err(crate::error::Error::Other(crate::error::OtherError{
            throw_site: throw_site!(),
            message: Option::<String>::None,
            source: Option::<crate::error::Error>::None,
        }))
    };
    ($msg:expr) => {
        Err(crate::error::Error::Other(crate::error::OtherError{
            throw_site: throw_site!(),
            message: Some($msg.into()),
            source: Option::<Box<dyn std::error::Error>>::None,
        }))
    };
    ($fmt:expr, $($arg:expr),+) => {
        Err(crate::error::Error::Other(crate::error::OtherError{
            throw_site: throw_site!(),
            message: Some(format!($fmt, $($arg),+)),
            source: Option::<Box<dyn std::error::Error>>::None,
        }))
    };
}

#[macro_export]
macro_rules! wrap {
    ($e:expr) => {
        Err(crate::error::Error::Other(crate::error::OtherError{
            throw_site: throw_site!(),
            message: Option::<String>::None,
            source: crate::error::box_err(Some($e)),
        }))
    };
    ($e:expr, $msg:expr) => {
        Err(crate::error::Error::Other(crate::error::OtherError{
            throw_site: throw_site!(),
            message: Some($msg.into()),
            source: crate::error::box_err(Some($e)),
        }))
    };
    ($e:expr, $fmt:expr, $($arg:expr),+) => {
        Err(crate::error::Error::Other(crate::error::OtherError{
            throw_site: throw_site!(),
            message: Some(format!($fmt, $($arg),+)),
            source: crate::error::box_err(Some($e)),
        }))
    };
}

/// Creates a ParseError object, requires an 'Iter' and the expected 'char'.
#[macro_export]
macro_rules! expect {
    ($iter:expr, $c:expr) => {
        $iter.expect($c, throw_site!())
    };
}

/// Creates a Result populated by a ParseError
/// iter: required as the first argument, `Iter`
/// message: optional, can be a string or a format
#[macro_export]
macro_rules! parse_err {
    // required: first argument must be the ParserState object
    ($iter:expr) => { Err(create_parser_error!(&$iter.st)) };
    // optional: second argument can be a simple string message
    ($iter:expr, $msg:expr) => { Err(create_parser_error!(&$iter.st, $msg) ) };
    // optional: format!
    ($iter:expr, $fmt:expr, $($arg:expr),+) => {
        Err(create_parser_error!(&$iter.st, $fmt, $($arg),+))
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
    let e = create_parser_error!(&p);
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
    let e = create_parser_error!(&p, message);
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
    let e = create_parser_error!(&p, "some message {}", 6);
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
    use crate::parser::Position;
    let iter = crate::parser::Iter {
        it: "".chars().peekable(),
        st: ParserState {
            position: Position {
                line: 2,
                column: 10,
                absolute: 31,
            },
            c: 'o',
            doc_status: Default::default(),
            tag_status: Default::default(),
        },
    };
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let result: crate::error::Result<u32> = parse_err!(iter);
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
    use crate::parser::Position;
    let iter = crate::parser::Iter {
        it: "".chars().peekable(),
        st: ParserState {
            position: Position {
                line: 2,
                column: 10,
                absolute: 31,
            },
            c: 'o',
            doc_status: Default::default(),
            tag_status: Default::default(),
        },
    };
    let message = "some message";
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let result: Result<Option<String>> = parse_err!(iter, message);
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
    use crate::parser::Position;
    use xdoc::ElementData;
    let iter = crate::parser::Iter {
        it: "".chars().peekable(),
        st: ParserState {
            position: Position {
                line: 5,
                column: 45,
                absolute: 9,
            },
            c: '🍺',
            doc_status: Default::default(),
            tag_status: Default::default(),
        },
    };
    let message = format!("some message {}", 6);
    let expected_file = file!().to_owned();
    let expected_line = line!() + 1;
    let result: Result<ElementData> = parse_err!(iter, "some message {}", 6);
    let e = result.err().unwrap();
    if let Error::Parse(pe) = e {
        assert_eq!(5, pe.xml_site.line);
        assert_eq!(9, pe.xml_site.position);
        assert_eq!(45, pe.xml_site.column);
        assert_eq!(expected_file, pe.throw_site.file);
        assert_eq!(expected_line, pe.throw_site.line);
        assert_eq!(message, pe.message.unwrap());
        assert_eq!('🍺', pe.xml_site.character);
    } else {
        panic!("wrong error type");
    }
}
