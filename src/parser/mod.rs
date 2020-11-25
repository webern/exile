/*!
This module is responsible for parsing XML from string representations.
!*/
use std::collections::HashMap;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

use crate::error::{OtherError, ThrowSite};
use crate::parser::bang::parse_bang;
use crate::parser::chars::{is_name_char, is_name_start_char};
use crate::parser::element::parse_element;
use crate::parser::error::{display_char, Result};
pub use crate::parser::error::{ParseError, XmlSite};
use crate::parser::pi::{parse_pi, parse_pi_logic};
use crate::{Declaration, Document, Encoding, Misc, Version};

#[macro_use]
mod macros;

mod bang;
mod chars;
mod element;
mod error;
mod pi;
mod string;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
pub(super) struct Position {
    pub line: u64,
    pub column: u64,
    pub absolute: u64,
}

impl Default for Position {
    fn default() -> Self {
        // These are the magic values needed to make the Position values 1-based.
        Position {
            line: 1,
            column: 1,
            absolute: 0, // this gets advanced when we start parsing (?)
        }
    }
}

impl Position {
    fn increment(&mut self, current_char: char) {
        self.absolute += 1;
        if current_char == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub(crate) struct ParserState {
    pub(crate) position: Position,
    pub(crate) c: char,
    pub(crate) doc_status: DocStatus,
    pub(crate) tag_status: TagStatus,
}

impl Default for ParserState {
    fn default() -> Self {
        Self {
            position: Default::default(),
            c: '_',
            doc_status: Default::default(),
            tag_status: Default::default(),
        }
    }
}

pub(crate) struct Iter<'a> {
    pub(crate) it: Peekable<Chars<'a>>,
    pub(crate) st: ParserState,
}

impl<'a> Iter<'a> {
    /// Returns an `Iter` primed with the first character, otherwise returns an error.
    fn new(s: &'a str) -> Result<Self> {
        let mut i = Iter {
            it: s.chars().peekable(),
            st: ParserState {
                position: Default::default(),
                c: '\0',
                doc_status: Default::default(),
                tag_status: Default::default(),
            },
        };
        if !i.advance() {
            return Err(ParseError {
                throw_site: throw_site!(),
                xml_site: None,
                message: Some("iter could not be initialized, empty document".to_string()),
                source: None,
            });
        }
        Ok(i)
    }

    /// Returns `false` if the iterator could not be advanced (end).
    pub(crate) fn advance(&mut self) -> bool {
        let option_char = self.it.next();
        match option_char {
            Some(c) => {
                self.st.c = c;
                self.st.position.increment(self.st.c);
                true
            }
            None => {
                // if we haven't already hit the end, the character will not be null. if that's the
                // case, we increment the position one more time to point past the end.
                if self.st.c != '\0' {
                    self.st.position.increment(self.st.c);
                }
                // set the character to a null so nobody reads the previous position's character
                self.st.c = '\0';
                false
            }
        }
    }

    pub(crate) fn advance_or_die(&mut self) -> Result<()> {
        if self.advance() {
            Ok(())
        } else {
            parse_err!(self, "iter could not be advanced")
        }
    }

    pub(crate) fn expect(&self, expected: char, site: ThrowSite) -> Result<()> {
        if self.is(expected) {
            Ok(())
        } else {
            Err(ParseError {
                throw_site: site,
                xml_site: Some(XmlSite::from_parser(&self.st)),
                message: Some(format!(
                    "expected '{}' but found '{}'",
                    display_char(expected),
                    display_char(self.st.c)
                )),
                source: None,
            })
        }
    }

    pub(crate) fn is_name_start_char(&self) -> bool {
        is_name_start_char(self.st.c)
    }

    pub(crate) fn is_name_char(&self) -> bool {
        is_name_char(self.st.c)
    }

    pub(crate) fn is_after_name_char(&self) -> bool {
        if self.is_whitespace() {
            return true;
        }
        matches!(self.st.c, ' ' | '\t' | '=' | '/' | '>' | '\n')
    }

    pub(crate) fn expect_name_start_char(&self) -> Result<()> {
        if self.is_name_start_char() {
            Ok(())
        } else {
            parse_err!(self, "expected name start char, found '{}'", self.st.c)
        }
    }

    pub(crate) fn expect_name_char(&self) -> Result<()> {
        if self.is_name_char() {
            Ok(())
        } else {
            parse_err!(self, "expected name char, found '{}'", self.st.c)
        }
    }

    pub(crate) fn skip_whitespace(&mut self) -> Result<()> {
        loop {
            if !self.is_whitespace() {
                return Ok(());
            }
            if !self.advance() {
                return Ok(());
            }
        }
    }

    pub(crate) fn is_whitespace(&self) -> bool {
        self.st.c.is_ascii_whitespace()
    }

    pub(crate) fn is(&self, value: char) -> bool {
        self.st.c == value
    }

    pub(crate) fn peek_is(&mut self, value: char) -> bool {
        if let Some(&next) = self.it.peek() {
            return next == value;
        }
        false
    }

    pub(crate) fn is_digit(&self) -> bool {
        self.st.c.is_ascii_digit()
    }

    pub(crate) fn is_hex(&self) -> bool {
        if self.is_digit() {
            return true;
        }
        (self.st.c >= 'A' && self.st.c <= 'F') || (self.st.c >= 'a' && self.st.c <= 'f')
    }

    // returns either the next char, or an error if the iter is at the end.
    pub(crate) fn peek_or_die(&mut self) -> Result<char> {
        let opt = self.it.peek();
        match opt {
            Some(c) => Ok(*c),
            None => parse_err!(self, "unexpected end of document"),
        }
    }

    /// Returns true if the character is `'\0'`, which means that the iter is exhausted.
    pub(super) fn end(&self) -> bool {
        self.st.c == '\0'
    }
}

pub(crate) fn document_from_string<S: AsRef<str>>(s: S) -> crate::error::Result<Document> {
    let mut iter = crate::parser::Iter::new(s.as_ref()).map_err(crate::error::Error::Parse)?;
    let mut document = Document::new();
    // TODO - this loop seems weird
    loop {
        parse_document(&mut iter, &mut document).map_err(crate::error::Error::Parse)?;
        if !iter.advance() {
            break;
        }
    }
    Ok(document)
}

pub(crate) fn document_from_file<P: AsRef<Path>>(path: P) -> crate::error::Result<Document> {
    let s = std::fs::read_to_string(path.as_ref()).map_err(|e| {
        crate::error::Error::Other(OtherError {
            throw_site: throw_site!(),
            message: Some(format!("Unable to read file '{}'", path.as_ref().display())),
            source: Some(Box::new(e)),
        })
    })?;
    document_from_string(s)
}

// TODO - disallow dead code
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
pub(crate) enum TagStatus {
    TagOpen(u64),
    InsideTag(u64),
    InsideProcessingInstruction(u64),
    TagClose(u64, u64),
    OutsideTag,
}

impl Default for TagStatus {
    fn default() -> Self {
        TagStatus::OutsideTag
    }
}

// TODO - disallow dead code
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
pub(crate) enum DocStatus {
    Declaration,
    Prolog,
    Root,
    Epilog,
}

impl Default for DocStatus {
    fn default() -> Self {
        DocStatus::Declaration
    }
}

fn parse_document(iter: &mut Iter<'_>, document: &mut Document) -> Result<()> {
    loop {
        if iter.st.c.is_ascii_whitespace() {
            if !iter.advance() {
                break;
            }
            continue;
        }
        expect!(iter, '<')?;
        let next = iter.peek_or_die()?;
        match next {
            '?' => match iter.st.doc_status {
                DocStatus::Declaration => {
                    parse_declaration_pi(iter, document)?;
                    iter.st.doc_status = DocStatus::Prolog
                }
                DocStatus::Prolog => {
                    let pi = parse_pi(iter)?;
                    document.push_prolog_misc(Misc::PI(pi));
                }
                DocStatus::Epilog => {
                    let pi = parse_pi(iter)?;
                    document.push_epilog_misc(Misc::PI(pi));
                }
                DocStatus::Root => {
                    return parse_err!(
                        iter,
                        "the parser state is inconsistent, should not be {:?}",
                        DocStatus::Root
                    );
                }
            },
            '!' => {
                let _ltparse = parse_bang(iter)?;
                // TODO - add it if appropriate
            }
            _ => {
                document.set_root(parse_element(iter)?);
                iter.st.doc_status = DocStatus::Epilog;
            }
        }

        if iter.end() {
            break;
        }
    }
    Ok(())
}

// takes the iter pointing to '<' and already expected to be '<?xml ...'. parses this and places
// the values found into the mutable document parameter
fn parse_declaration_pi(iter: &mut Iter<'_>, document: &mut Document) -> Result<()> {
    state_must_be_before_declaration(iter)?;
    let (target, data) = parse_pi_logic(iter)?;
    document.set_declaration(parse_declaration(iter, &target, &data)?);
    Ok(())
}

/// Given the target and data from the declaration processing instruction, parse the XML version and
/// encoding. For example. `iter` is only passed to make error construction easier.
fn parse_declaration(iter: &Iter<'_>, target: &str, data: &str) -> Result<Declaration> {
    let mut declaration = Declaration::default();
    if target != "xml" {
        return parse_err!(iter, "pi_data.target != xml");
    }
    let instructions: Vec<&str> = data.split_whitespace().collect();
    if instructions.len() > 2 {
        return parse_err!(
            iter,
            "only able to parse xml declarations that include version and encoding. \
        a string split of the xml processing instruction data yielded more than two items."
        );
    }
    let map = parse_as_map(iter, &instructions)?;
    if let Some(&val) = map.get("version") {
        match val {
            "1.0" => {
                declaration.version = Some(Version::V10);
            }
            "1.1" => {
                declaration.version = Some(Version::V11);
            }
            _ => {
                return parse_err!(iter, "unknown or unsupported XML version number '{}'", val);
            }
        }
    }
    if let Some(&val) = map.get("encoding") {
        match val {
            "UTF-8" => {
                declaration.encoding = Some(Encoding::Utf8);
            }
            _ => {
                return parse_err!(iter, "unknown or unsupported encoding string '{}'", val);
            }
        }
    }
    Ok(declaration)
}

/// Splits each string on '=', then for the value on the right, expects it to be a quoted string
/// starting with either `"` or `'`. `iter` is only passed to ease error construction.
fn parse_as_map<'a, S: AsRef<str>>(
    iter: &Iter<'_>,
    data: &'a [S],
) -> Result<HashMap<&'a str, &'a str>> {
    let mut result = HashMap::new();
    for item in data {
        let s = item.as_ref();
        let split = s.split('=').collect::<Vec<&str>>();
        match split.len() {
            0 => continue,
            1 => {
                result.insert(*split.first().unwrap(), "");
            }
            _ => {
                let quoted_value = *split.get(1).unwrap();
                let len = quoted_value.len();
                if len < 2 {
                    return parse_err!(
                        iter,
                        "unparseable string encountered in XML declaration: '{}'",
                        quoted_value
                    );
                }
                let open = &quoted_value[..1];
                let middle = &quoted_value[1..len - 1];
                let end = &quoted_value[len - 1..];
                if (open != "'" && open != "\"") || open != end {
                    return parse_err!(
                        iter,
                        "bad quotation marks encountered in XML declaration: '{}' and '{}'",
                        open,
                        end
                    );
                }
                result.insert(*split.first().unwrap(), middle);
            }
        }
    }
    Ok(result)
}

fn state_must_be_before_declaration(iter: &Iter<'_>) -> Result<()> {
    if iter.st.doc_status != DocStatus::Declaration {
        return parse_err!(iter, "state_must_be_before_declaration");
    } else {
        Ok(())
    }
}

fn parse_name(iter: &mut Iter<'_>) -> Result<String> {
    iter.expect_name_start_char()?;
    let mut name = String::default();
    name.push(iter.st.c);
    iter.advance_or_die()?;
    loop {
        if iter.is_after_name_char() {
            break;
        }
        iter.expect_name_char()?;
        name.push(iter.st.c);
        if !iter.advance() {
            break;
        }
    }
    Ok(name)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn xml_00() {
    let xml = "<doc/>";
    let doc = document_from_string(xml).unwrap();
    assert!(doc.declaration().version.is_none());
    assert!(doc.declaration().encoding.is_none());
    assert_eq!(0, doc.root().nodes_len());
    assert_eq!("doc", doc.root().fullname());
}

#[test]
fn xml_01() {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?><doc/>"#;
    let doc = document_from_string(xml).unwrap();
    assert_eq!(Version::V10, doc.declaration().version.unwrap());
    assert_eq!(Encoding::Utf8, doc.declaration().encoding.unwrap());
    assert_eq!(0, doc.root().nodes_len());
    assert_eq!("doc", doc.root().fullname());
}

#[test]
fn xml_02() {
    let xml = r#"<?xml encoding="UTF-8"?><doc/>"#;
    let doc = document_from_string(xml).unwrap();
    assert!(doc.declaration().version.is_none());
    assert_eq!(Encoding::Utf8, doc.declaration().encoding.unwrap());
    assert_eq!(0, doc.root().nodes_len());
    assert_eq!("doc", doc.root().fullname());
}

#[test]
fn xml_03() {
    let xml = r#"<?xml version="1.0"?><doc/>"#;
    let doc = document_from_string(xml).unwrap();
    assert_eq!(Version::V10, doc.declaration().version.unwrap());
    assert!(doc.declaration().encoding.is_none());
    assert_eq!(0, doc.root().nodes_len());
    assert_eq!("doc", doc.root().fullname());
}

#[cfg(test)]
mod tests {
    const _XML1: &str = r##"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE something PUBLIC "-//Some//Path//EN" "http://www.example.org/dtds/partwise.dtd">
<cats>
  <cat id="b1">
    <name>
        Bones
    </name>
  <birthdate>2008-06-01</birthdate>
  </cat>
  <cat id="b2">
    <name>Bishop</name>
    <birthdate>2012-01-01</birthdate>
  </cat>
</cats>
    "##;
}
