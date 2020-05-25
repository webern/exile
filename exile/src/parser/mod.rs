use std::iter::Peekable;
use std::str::Chars;

use xdoc::{Declaration, Document, Encoding, PIData, Version};

use crate::error::{parse_err, Error, ParseError, Result, ThrowSite, XMLSite};
use crate::parser::chars::{is_name_char, is_name_start_char};
use crate::parser::element::parse_element;
use crate::parser::pi::parse_pi;

mod chars;
mod element;
mod pi;
mod string;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
pub struct Position {
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
                c: 'x',
                doc_status: Default::default(),
                tag_status: Default::default(),
            },
        };
        if !i.advance() {
            return Err(Error::Parse(ParseError {
                throw_site: ThrowSite {
                    file: file!().to_owned(),
                    line: line!(),
                },
                xml_site: XMLSite::from_parser(&ParserState::default()),
                message: Some("iter advancement was required, but not possible".to_string()),
                source: None,
            }));
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
            None => false,
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
            Err(parse_err(
                &self.st,
                site,
                Some(format!("expected '{}' but found '{}'", expected, self.st.c)),
                Option::<Error>::None,
            ))
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
        match self.st.c {
            ' ' | '\t' | '=' | '/' | '>' | '\n' => true,
            _ => false,
        }
    }

    pub(crate) fn expect_name_start_char(&self) -> Result<()> {
        if self.is_name_start_char() {
            Ok(())
        } else {
            // TODO(https://github.com/webern/exile/issues/8)  - this error occurs if a comment or pi is encountered
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
}

pub fn parse_str(s: &str) -> Result<Document> {
    let mut iter = Iter::new(s)?;
    let mut document = Document::new();
    loop {
        parse_document(&mut iter, &mut document)?;
        if !iter.advance() {
            break;
        }
    }
    Ok(document)
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
    BeforeDeclaration,
    Prologue,
    Root,
    Trailing,
}

impl Default for DocStatus {
    fn default() -> Self {
        DocStatus::BeforeDeclaration
    }
}
/*

prolog	   ::=   	XMLDecl Misc* (doctypedecl Misc*)?
Misc	   ::=   	Comment | PI | S (S is whitespace)
*/

fn parse_document(iter: &mut Iter, document: &mut Document) -> Result<()> {
    loop {
        if iter.st.c.is_ascii_whitespace() {
            if !iter.advance() {
                break;
            }
            continue;
        }
        expect!(iter, '<')?;
        let next = peek_or_die(iter)?;
        match next {
            '?' => match iter.st.doc_status {
                DocStatus::BeforeDeclaration => parse_declaration_pi(iter, document)?,
                DocStatus::Prologue | DocStatus::Root | DocStatus::Trailing => {
                    skip_processing_instruction(iter)?
                }
            },
            '!' => {
                iter.advance_or_die()?;
                if iter.peek_is('-') {
                    skip_comment(iter)?;
                } else {
                    skip_doctype(iter)?
                }
            }
            _ => {
                document.root = parse_element(iter)?;
            }
        }

        if !iter.advance() {
            break;
        }
    }
    Ok(())
}

fn parse_declaration_pi(iter: &mut Iter, document: &mut Document) -> Result<()> {
    state_must_be_before_declaration(iter)?;
    let pi_data = parse_pi(iter)?;
    document.declaration = parse_declaration(&pi_data)?;
    iter.st.doc_status = DocStatus::Prologue;
    Ok(())
}

fn parse_declaration(pi_data: &PIData) -> Result<Declaration> {
    let mut declaration = Declaration::default();
    if pi_data.target != "xml" {
        return raise!("pi_data.target != xml");
    }
    if pi_data.instructions.map().len() > 2 {
        return raise!("");
    }
    if let Some(val) = pi_data.instructions.map().get("version") {
        match val.as_str() {
            "1.0" => {
                declaration.version = Version::One;
            }
            "1.1" => {
                declaration.version = Version::OneDotOne;
            }
            _ => {
                return raise!("");
            }
        }
    }
    if let Some(val) = pi_data.instructions.map().get("encoding") {
        match val.as_str() {
            "UTF-8" => {
                declaration.encoding = Encoding::Utf8;
            }
            _ => {
                return raise!("");
            }
        }
    }
    Ok(declaration)
}

fn state_must_be_before_declaration(iter: &Iter) -> Result<()> {
    if iter.st.doc_status != DocStatus::BeforeDeclaration {
        return raise!("");
    } else {
        Ok(())
    }
}

pub(crate) fn peek_or_die(iter: &mut Iter) -> Result<char> {
    let opt = iter.it.peek();
    match opt {
        Some(c) => Ok(*c),
        None => raise!(""),
    }
}

fn parse_name(iter: &mut Iter) -> Result<String> {
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

pub(crate) fn skip_doctype(iter: &mut Iter) -> Result<()> {
    expect!(iter, '!')?;
    while !iter.is('>') {
        if iter.is('[') {
            skip_nested_doctype_stuff(iter)?
        }
        iter.advance_or_die()?;
    }
    Ok(())
}

pub(crate) fn skip_comment(iter: &mut Iter) -> Result<()> {
    expect!(iter, '!')?;
    iter.advance_or_die()?;
    expect!(iter, '-')?;
    iter.advance_or_die()?;
    expect!(iter, '-')?;
    iter.advance_or_die()?;
    let mut consecutive_dashes: u8 = 0;
    loop {
        if iter.is('-') {
            consecutive_dashes += 1;
        } else if iter.is('>') && consecutive_dashes == 2 {
            break;
        } else {
            consecutive_dashes = 0;
        }
        iter.advance_or_die()?;
    }
    Ok(())
}

pub(crate) fn skip_nested_doctype_stuff(iter: &mut Iter) -> Result<()> {
    expect!(iter, '[')?;
    iter.advance_or_die()?;
    while !iter.is(']') {
        iter.advance_or_die()?;
    }
    Ok(())
}

pub(crate) fn skip_processing_instruction(iter: &mut Iter) -> Result<()> {
    expect!(iter, '<')?;
    iter.advance_or_die()?;
    expect!(iter, '?')?;
    iter.advance_or_die()?;
    while !iter.is('?') {
        iter.advance_or_die()?;
    }
    iter.advance_or_die()?;
    expect!(iter, '>')?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

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
