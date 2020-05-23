extern crate env_logger;

use std::io::prelude::*;
use std::str::Chars;

use snafu::{Backtrace, GenerateBacktrace, ResultExt};

use xdoc::Document;

use crate::error::{self, Result};
use crate::parser::TagStatus::OutsideTag;

// Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// Clone, to create T from &T via a copy.
// Copy, to give a type 'copy semantics' instead of 'move semantics'.
// Hash, to compute a hash from &T.
// Default, to create an empty instance of a data type.
// Debug, to format a value using the {:?} formatter.
// #[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]

const _BUFF_SIZE: usize = 1024;

pub fn _parse<R: BufRead>(r: &mut R) -> error::Result<Document> {
    let mut s = String::new();
    let _ = r.read_to_string(&mut s).context(error::IoRead {
        parse_location: error::ParseLocation { line: 0, column: 0 },
    })?;
    parse_str(&s)
}

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

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash, Default)]
struct ParserState {
    position: Position,
    // doc_state: DocState,
    current_char: char,
    tag_status: TagStatus,
}

pub fn parse_str(s: &str) -> Result<Document> {
    let mut state = ParserState {
        position: Default::default(),
        // doc_state: DocState::BeforeFirstTag,
        current_char: '\0',
        tag_status: OutsideTag,
    };

    let mut iter = s.chars();
    while advance_parser(&mut iter, &mut state) {
        let _state = format!("{:?}", state);
        process_char(&mut iter, &mut state)?;
        trace!("{:?}", state);
    }

    Ok(Document::new())
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
enum TagStatus {
    TagOpen(u64),
    InsideTag(u64),
    TagClose(u64, u64),
    OutsideTag,
}

impl Default for TagStatus {
    fn default() -> Self {
        TagStatus::OutsideTag
    }
}

fn is_space_or_alpha(c: char) -> bool {
    c.is_alphabetic() || c.is_ascii_whitespace()
}

fn is_pi_indicator(c: char) -> bool {
    c == '?' || c == '!'
}

fn process_char(_iter: &mut Chars, state: &mut ParserState) -> Result<()> {
    let _state_str = format!("{:?}", state);
    match state.tag_status {
        TagStatus::TagOpen(pos) => {
            if state.current_char != '/'
                && !is_space_or_alpha(state.current_char)
                && !is_pi_indicator(state.current_char)
            {
                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
            state.tag_status = TagStatus::InsideTag(pos)
        }
        TagStatus::InsideTag(pos) => {
            if state.current_char == '>' {
                state.tag_status = TagStatus::TagClose(pos, state.position.absolute)
            } else if state.current_char == '<' {
                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
        TagStatus::TagClose(_start, _end) => {
            if state.current_char == '<' {
                state.tag_status = TagStatus::TagOpen(state.position.absolute);
            } else if state.current_char == '>' {
                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            } else {
                state.tag_status = TagStatus::OutsideTag;
            }
            // TODO pop the start and stop locations over to a tag parser?
        }
        OutsideTag => {
            if state.current_char == '<' {
                state.tag_status = TagStatus::TagOpen(state.position.absolute);
            } else if state.current_char == '>' {
                return Err(error::Error::Parse {
                    position: state.position,
                    backtrace: Backtrace::generate(),
                });
            }
        }
    }
    Ok(())
}

fn advance_parser(iter: &mut Chars<'_>, state: &mut ParserState) -> bool {
    let option_char = iter.next();
    match option_char {
        Some(c) => {
            state.current_char = c;
            state.position.increment(state.current_char);
            true
        }
        None => false,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    const XML1: &str = r##"
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

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // Check if a url with a trailing slash and one without trailing slash can both be parsed
    #[test]
    fn parse_a_doo_dah() {
        init_logger();
        let the_thing = XML1;
        let _ = parse_str(the_thing).unwrap();
    }
}
