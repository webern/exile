use std::borrow::Cow;

use crate::error::Result;
use crate::parser::Iter;

pub(crate) fn parse_string(iter: &mut Iter, end_char: char) -> Result<String> {
    let mut result = String::new();
    while iter.st.c != end_char {
        if iter.st.c == '&' {
            let c = parse_escape(iter)?;
            result.push(c);
        } else {
            result.push(iter.st.c);
        }
        if !iter.advance() {
            break;
        }
    }
    Ok(result)
}

/*
amp   &
apos  '
gt    >
lt    <
quot  "
 */
fn parse_escape(iter: &mut Iter) -> Result<char> {
    iter.advance_or_die()?;
    match iter.st.c {
        'a' => parse_amp_or_apos(iter),
        'g' => parse_gt(iter),
        'l' => parse_lt(iter),
        'q' => parse_quot(iter),
        '#' => parse_codepoint(iter),
        _ => parse_err!(iter, "unexpected character in escape sequence"),
    }
}

fn parse_amp_or_apos(iter: &mut Iter) -> Result<char> {
    Ok('1')
}
fn parse_amp(iter: &mut Iter) -> Result<char> {
    Ok('2')
}
fn parse_apos(iter: &mut Iter) -> Result<char> {
    Ok('3')
}
fn parse_gt(iter: &mut Iter) -> Result<char> {
    Ok('4')
}
fn parse_lt(iter: &mut Iter) -> Result<char> {
    Ok('5')
}
fn parse_quot(iter: &mut Iter) -> Result<char> {
    Ok('6')
}
fn parse_codepoint(iter: &mut Iter) -> Result<char> {
    Ok('6')
}
