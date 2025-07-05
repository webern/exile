use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use crate::error::ThrowSite;
use crate::parser::ParserState;

/// Alias for `Result<T, Error>`.
pub(crate) type Result<T> = std::result::Result<T, ParseError>;

/// Represents an error that occurred during parsing because the XML document is not well-formed.
#[derive(Debug, Default)]
pub struct ParseError {
    /// The location in this library's sourcecode where the error was thrown.
    pub throw_site: ThrowSite,
    /// The location in the XML file where the syntax error was encountered.
    pub xml_site: Option<XmlSite>,
    /// An optional error message.
    pub message: Option<String>,
    /// An optional underlying error (i.e. an optional wrapped error)
    pub source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

/// The position in the XML file that the parser was at when the error was thrown. Ideally this
/// will match the exact location where an XML file first violates XML syntax. These numbers are
/// 1-based, i.e. line 1 is the first first line of the file, column 1 is the leftmost character of
/// a line, and position 1 is the first character of the document.
#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct XmlSite {
    /// The line in the XML file where an error was encountered. 1-based indexing.
    pub line: u64,
    /// The character within the line where an error was encountered. 1-based indexing.
    pub column: u64,
    /// The absolute character position within the line where an error was encountered. 1-based.
    pub position: u64,
    /// The character that was in scope with the error was encountered.
    pub character: char,
}

pub(super) fn display_char(c: char) -> String {
    match c {
        '\n' => "\\n".into(),
        '\t' => "\\t".into(),
        '\r' => "\\r".into(),
        '\u{000B}' => "\\u000B".into(),
        '\u{000C}' => "\\u000C".into(),
        ' ' => "%20".into(),
        _ => format!("{c}"),
    }
}

impl Display for XmlSite {
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

impl XmlSite {
    pub(crate) fn from_parser(p: &ParserState) -> Self {
        Self {
            line: p.position.line,
            column: p.position.column,
            position: p.position.absolute,
            character: p.c,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.throw_site, f)?;
        if let Some(xml_site) = &self.xml_site {
            write!(f, " xml ")?;
            Display::fmt(&xml_site, f)?;
        }
        if let Some(msg) = &self.message {
            if !msg.is_empty() {
                write!(f, " - {msg}")?;
            }
        }
        if let Some(e) = &self.source {
            write!(f, " - caused by: ")?;
            Display::fmt(&e, f)?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &dyn std::error::Error)
    }
}

pub(super) fn parse_err<S, E>(
    parser_state: &ParserState,
    throw_site: ThrowSite,
    message: Option<S>,
    source: Option<E>,
) -> crate::parser::error::ParseError
where
    S: Into<String>,
    E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
{
    crate::parser::error::ParseError {
        throw_site,
        xml_site: Some(XmlSite::from_parser(parser_state)),
        message: message.map(|s| s.into()),
        source: source.map(|e| e.into()),
    }
}
