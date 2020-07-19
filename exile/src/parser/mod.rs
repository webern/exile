use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use xdoc::{Declaration, Document, Encoding, Misc, Version};

use crate::error::{display_char, parse_err, Error, ParseError, Result, ThrowSite, XMLSite};
use crate::parser::chars::{is_name_char, is_name_start_char};
use crate::parser::element::parse_element;
use crate::parser::pi::{parse_pi, parse_pi_logic};

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
                Some(format!(
                    "expected '{}' but found '{}'",
                    display_char(expected),
                    display_char(self.st.c)
                )),
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
            None => raise!(""),
        }
    }
}

pub(crate) fn document_from_string(s: &str) -> crate::error::Result<Document> {
    let mut iter = crate::parser::Iter::new(s)?;
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
                    return raise!(
                        "the parser state is inconsistent, should not be {:?}",
                        DocStatus::Root
                    );
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
                document.set_root(parse_element(iter)?);
                iter.st.doc_status = DocStatus::Epilog;
            }
        }

        if !iter.advance() {
            break;
        }
    }
    Ok(())
}

// takes the iter pointing to '<' and already expected to be '<?xml ...'. parses this and places
// the values found into the mutable document parameter
fn parse_declaration_pi(iter: &mut Iter<'_>, document: &mut Document) -> Result<()> {
    state_must_be_before_declaration(iter)?;
    let (target, instructions) = parse_pi_logic(iter)?;
    document.set_declaration(parse_declaration(&target, &instructions)?);
    Ok(())
}

fn parse_declaration(target: &str, instructions: &Vec<String>) -> Result<Declaration> {
    let mut declaration = Declaration::default();
    if target != "xml" {
        return raise!("pi_data.target != xml");
    }
    if instructions.len() > 2 {
        return raise!("");
    }
    let map = parse_as_map(instructions);
    if let Some(&val) = map.get("version") {
        match val {
            "\"1.0\"" => {
                declaration.version = Version::One;
            }
            "\"1.1\"" => {
                declaration.version = Version::OneDotOne;
            }
            _ => {
                return raise!("");
            }
        }
    }
    if let Some(&val) = map.get("encoding") {
        match val {
            "\"UTF-8\"" => {
                declaration.encoding = Encoding::Utf8;
            }
            _ => {
                return raise!("");
            }
        }
    }
    Ok(declaration)
}

fn parse_as_map<'a, S: AsRef<str>>(data: &'a [S]) -> HashMap<&'a str, &'a str> {
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
                result.insert(*split.first().unwrap(), *split.get(1).unwrap());
            }
        }
    }
    result
}

fn state_must_be_before_declaration(iter: &Iter<'_>) -> Result<()> {
    if iter.st.doc_status != DocStatus::Declaration {
        return raise!("");
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

// takes the iter after a '<' and when it is pointing at a '!'. returns when '-->' is encountered.
// will not work if the node being parsed is a DOCTYPE, you must already know it to be a comment.
// TODO - support comments https://github.com/webern/exile/issues/27
pub(crate) fn skip_comment(iter: &mut Iter<'_>) -> Result<()> {
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

// takes the iter after a '<' and when it is pointing at a '!'. returns when '>' is encountered.
// will not work if the node being parsed is a comment, you must already know it to be a DOCTYPE
// TODO - support doctypes https://github.com/webern/exile/issues/22
pub(crate) fn skip_doctype(iter: &mut Iter<'_>) -> Result<()> {
    expect!(iter, '!')?;
    while !iter.is('>') {
        if iter.is('[') {
            skip_nested_doctype_stuff(iter)?
        }
        iter.advance_or_die()?;
    }
    Ok(())
}

// takes the iter when it is inside if a <!DOCTYPE construct and has encountered the '[' char.
// ignores everything and returns the iter when it is pointing to the first encountered ']'
// TODO - support doctypes https://github.com/webern/exile/issues/22
pub(crate) fn skip_nested_doctype_stuff(iter: &mut Iter<'_>) -> Result<()> {
    expect!(iter, '[')?;
    iter.advance_or_die()?;
    while !iter.is(']') {
        iter.advance_or_die()?;
    }
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
