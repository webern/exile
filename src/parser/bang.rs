/*!
The `bang` module parses those constructs that start with `<!`.
!*/

use crate::Node;

use super::element::LTParse;
use super::Iter;
use super::Result;

pub(super) fn parse_bang(iter: &mut Iter<'_>) -> Result<LTParse> {
    debug_assert_eq!('<', iter.st.c);
    iter.advance_or_die()?;
    debug_assert_eq!('!', iter.st.c);
    let next = iter.peek_or_die()?;
    match next {
        '-' => {
            // skip comment expects the iter to be advanced passed lt
            skip_comment(iter)?;
            Ok(LTParse::Skip)
        }
        '[' => {
            let cdata = parse_cdata(iter)?;
            Ok(LTParse::Some(Node::CData(cdata)))
        }
        'D' => {
            skip_doctype(iter)?;
            Ok(LTParse::Skip)
        }
        _ => return parse_err!(iter, "illegal char '{}' after <!", iter.st.c),
    }
}

// takes the iter when it is pointing at a '!'. returns when '-->' is encountered. Returns an error
// if it is not a well-formed comment.
// TODO - support comments https://github.com/webern/exile/issues/27
pub(super) fn skip_comment(iter: &mut Iter<'_>) -> Result<()> {
    expect!(iter, '!')?;
    iter.advance_or_die()?;
    expect!(iter, '-')?;
    iter.advance_or_die()?;
    expect!(iter, '-')?;
    iter.advance_or_die()?;
    loop {
        if iter.is('-') && iter.peek_is('-') {
            // advance to the second dash
            iter.advance_or_die()?;
            // advance to the char after the second dash
            iter.advance_or_die()?;
            if iter.is('>') {
                break;
            } else {
                return parse_err!(iter, "-- is not allowed in a comment string");
            }
        }
        iter.advance_or_die()?;
    }
    // advance the iter to the char following -->
    iter.advance();
    Ok(())
}

// takes the iter after a '<' and when it is pointing at a '!'. returns when '>' is encountered.
// will not work if the node being parsed is a comment, you must already know it to be a DOCTYPE
// TODO - support doctypes https://github.com/webern/exile/issues/22
pub(super) fn skip_doctype(iter: &mut Iter<'_>) -> Result<()> {
    expect!(iter, '!')?;
    while !iter.is('>') {
        if iter.is('[') {
            skip_nested_doctype_stuff(iter)?
        }
        iter.advance_or_die()?;
    }
    // advance the iter to the char following ]>
    iter.advance();
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

/// `<![CDATA[foo]]>`
fn parse_cdata(iter: &mut Iter<'_>) -> Result<String> {
    // we assume the first char has been checked and is !
    iter.advance_or_die()?;
    // we assume the second char has been checked and is [
    iter.advance_or_die()?;
    expect!(iter, 'C')?;
    iter.advance_or_die()?;
    expect!(iter, 'D')?;
    iter.advance_or_die()?;
    expect!(iter, 'A')?;
    iter.advance_or_die()?;
    expect!(iter, 'T')?;
    iter.advance_or_die()?;
    expect!(iter, 'A')?;
    iter.advance_or_die()?;
    expect!(iter, '[')?;
    iter.advance_or_die()?;
    let mut data = String::new();
    let mut backet_buffer = String::new();
    loop {
        let c = iter.st.c;
        if c == ']' {
            backet_buffer.push(']');
            iter.advance_or_die()?;
            continue;
        } else if c == '>' && backet_buffer.len() >= 2 {
            if backet_buffer.len() > 2 {
                data.push_str(&backet_buffer[2..])
            }
            break;
        } else if !backet_buffer.is_empty() {
            data.push_str(&backet_buffer);
            backet_buffer.clear();
        }
        data.push(c);
        iter.advance_or_die()?
    }
    iter.advance();
    Ok(data)
}

#[test]
fn parse_bang_cdata_1() {
    let data = "foo";
    let iter_char_after = 'b';
    let input = format!("<![CDATA[{}]]>bar", data);
    let expected = LTParse::Some(Node::CData(data.into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_cdata_2() {
    let data = "foo]] >bar]>]>x";
    let iter_char_after = 'x';
    let input = format!("<![CDATA[{}]]>x", data);
    let expected = LTParse::Some(Node::CData(data.into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_cdata_3() {
    let data = "foo]]>bar]>]>x";
    let iter_char_after = 'b';
    let input = format!("<![CDATA[{}]]>x", data);
    let expected = LTParse::Some(Node::CData("foo".into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_cdata_4() {
    let data = "<xml>bloop</xml>";
    let iter_char_after = '<';
    let input = format!("<![CDATA[{}]]><foo></foo>", data);
    let expected = LTParse::Some(Node::CData(data.into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_cdata_5() {
    let data = "<![CDATA[";
    let iter_char_after = '<';
    let input = format!("<![CDATA[{}]]><foo></foo>", data);
    let expected = LTParse::Some(Node::CData(data.into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_cdata_6() {
    let data = "<&]>]";
    let iter_char_after = 'b';
    let input = format!("<![CDATA[{}]]>bar", data);
    let expected = LTParse::Some(Node::CData(data.into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_cdata_7() {
    let data = "]";
    let iter_char_after = 'b';
    let input = format!("<![CDATA[{}]]>bar", data);
    let expected = LTParse::Some(Node::CData(data.into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_cdata_8() {
    let data = "]]";
    let iter_char_after = 'b';
    let input = format!("<![CDATA[{}]]>bar", data);
    let expected = LTParse::Some(Node::CData(data.into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_cdata_9() {
    let data = "]]]";
    let iter_char_after = 'b';
    let input = format!("<![CDATA[{}]]>bar", data);
    let expected = LTParse::Some(Node::CData(data.into()));
    let mut iter = Iter::new(&input).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

// TODO - write real tests for doctypes https://github.com/webern/exile/issues/22
#[test]
fn parse_bang_doctype() {
    let data = r#"<!DOCTYPE doc [
<!ELEMENT doc (#PCDATA)>
<!ATTLIST doc a1 CDATA #IMPLIED>
]>x"#;
    let iter_char_after = 'x';
    let expected = LTParse::Skip;
    let mut iter = Iter::new(data).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

// TODO - write real tests for comments https://github.com/webern/exile/issues/27
#[test]
fn parse_bang_comment_1() {
    let data = r#"<!-- foo -->x"#;
    let iter_char_after = 'x';
    let expected = LTParse::Skip;
    let mut iter = Iter::new(data).unwrap();
    let actual = parse_bang(&mut iter).unwrap();
    assert_eq!(expected, actual);
    assert_eq!(iter_char_after, iter.st.c);
}

#[test]
fn parse_bang_comment_2() {
    let data = r#"<!-- -- -->x"#;
    let mut iter = Iter::new(data).unwrap();
    let result = parse_bang(&mut iter);
    assert!(result.is_err());
}
