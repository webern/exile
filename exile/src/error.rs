// Copyright 2020 by Matthew James Briggs

#![allow(clippy::default_trait_access)]

use core::fmt;
use std::fmt::{Display, Formatter};

use crate::parser::ParserState;

////////////////////////////////////////////////////////////////////////////////////////////////////
// public error type
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Alias for `Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for this library.
#[derive(Debug)]
pub enum Error {
    /// A syntax error encountered when parsing an XML document.
    Parse(ParseError),
    /// Any other error not related to the syntax of the XML document.
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
    }
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

pub(crate) fn display_char(c: char) -> String {
    match c {
        '\n' => "\\n".into(),
        '\t' => "\\t".into(),
        '\r' => "\\r".into(),
        '\u{000B}' => "\\u000B".into(),
        '\u{000C}' => "\\u000C".into(),
        ' ' => "%20".into(),
        _ => format!("{}", c),
    }
}

impl Display for XMLSite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "position: {}, line: {}, column: {}, character: '{}'",
            self.position,
            self.line,
            self.column,
            display_char(self.character)
        )
    }
}

/// Represents an error that occurred during parsing because the XML document is not well-formed.
#[derive(Debug, Default)]
pub struct ParseError {
    /// The location in this library's sourcecode where the error was thrown.
    pub throw_site: ThrowSite,
    /// The location in the XML file where the syntax error was encountered.
    pub xml_site: XMLSite,
    /// An optional error message.
    pub message: Option<String>,
    /// An optional underlying error (i.e. an optional wrapped error)
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

/// Represents any error that is not related to the syntax of the XML file.
#[derive(Debug, Default)]
pub struct OtherError {
    /// The location in this library's sourcecode where the error was thrown.
    pub throw_site: ThrowSite,
    /// An optional error message.
    pub message: Option<String>,
    /// An optional underlying error that is being wrapped.
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// private
////////////////////////////////////////////////////////////////////////////////////////////////////

impl XMLSite {
    pub(crate) fn from_parser(p: &ParserState) -> Self {
        Self {
            line: p.position.line,
            column: p.position.column,
            position: p.position.absolute,
            character: p.c,
        }
    }
}

pub(crate) fn box_err<E>(err: Option<E>) -> Option<Box<dyn std::error::Error>>
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// internal macros
////////////////////////////////////////////////////////////////////////////////////////////////////

/// This macro is used internally to obtain the current file and line (in the sourcecode).
#[macro_export]
macro_rules! throw_site {
    () => {
        crate::error::ThrowSite {
            file: file!().to_owned(),
            line: line!(),
        }
    };
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

/// This macro is used internally to create an `Err(crate::error::Error)`.
/// The `iter` is always required as the first argument, the second+ arguments are for format!()
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

/// This macro is used internally to wrap a foreign Result type into a `crate::error::Result`.
/// The first argument is always a `Result`, and the second+ arguments are for format!()
#[macro_export]
macro_rules! wrap {
    ($e:expr) => {
        match $e {
            Ok(value) => Ok(value),
            Err(er) => {
                Err(crate::error::Error::Other(crate::error::OtherError{
                    throw_site: throw_site!(),
                    message: Some("error".into()),
                    source: crate::error::box_err(Some(er)),
                }))
            }
        }
    };
    ($e:expr, $msg:expr) => {
        match $e {
            Ok(value) => Ok(value),
            Err(er) => {
                Err(crate::error::Error::Other(crate::error::OtherError{
                    throw_site: throw_site!(),
                    message: Some($msg.into()),
                    source: crate::error::box_err(Some(er)),
                }))
            }
        }
    };
    ($e:expr, $fmt:expr, $($arg:expr),+) => {
        match $e {
            Ok(value) => Ok(value),
            Err(er) => {
                Err(crate::error::Error::Other(crate::error::OtherError{
                    throw_site: throw_site!(),
                    message: Some(format!($fmt, $($arg),+)),
                    source: crate::error::box_err(Some(er)),
                }))
            }
        }
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// tests
////////////////////////////////////////////////////////////////////////////////////////////////////

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
    use xdoc::Element;
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
    let result: Result<Element> = parse_err!(iter, "some message {}", 6);
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

#[test]
fn wrap_macro() {
    let file = file!();
    let line = line!() + 1;
    let e = wrap!(std::fs::read_to_string("bad path 🍺🍺🍺"))
        .err()
        .unwrap();
    if let Error::Other(oe) = e {
        let display = format!("{}", oe);
        let expected = format!(
            "{}:{} - error - caused by: No such file or directory (os error 2)",
            file, line
        );
        assert_eq!(expected, display)
    } else {
        panic!("wrong error type");
    }
}
