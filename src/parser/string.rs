use crate::error::Result;
use crate::parser::Iter;
use crate::xdoc::is_whitespace;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub(crate) enum StringType {
    /// A string inside of an element, e.g. <element>string</element>.
    /// TODO - things does not work for <element>some<p/>string</element>
    Element,
    /// An attribute value string surrounded by double quotes, e.g. `"value"`.
    AttributeDouble,
    /// An attribute value string surrounded by single quotes, e.g. `'value'`.
    AttributeSingle,
    // TODO - support CDATA https://github.com/webern/exile/issues/28
    // CDATA,
}

/// Parse a string that terminates based on some character(s) determined by `string_type`.
pub(crate) fn parse_string(iter: &mut Iter<'_>, string_type: StringType) -> Result<String> {
    let mut space_buffer = None;
    let mut is_non_white_reached = false;
    let mut result = String::new();
    while !is_end_char(iter, string_type) {
        if iter.st.c == '&' {
            let c = parse_escape(iter)?;
            if is_whitespace(c) {
                if is_non_white_reached {
                    space_buffer = Some(' ');
                }
            } else {
                is_non_white_reached = true;
                result.push(c);
            }
        } else if is_forbidden(iter, string_type) {
            return parse_err!(iter, "forbidden character in {:?} string", string_type);
        } else if iter.is_whitespace() {
            if is_non_white_reached {
                space_buffer = Some(' ');
            }
        } else {
            is_non_white_reached = true;
            if let Some(space) = space_buffer {
                result.push(space);
                space_buffer = None;
            }
            result.push(iter.st.c);
        }

        if !iter.advance() {
            return parse_err!(
                iter,
                "input ended before termination character '{}' was reached",
                end_char(string_type)
            );
        }
    }
    Ok(result)
}

fn is_forbidden(iter: &Iter<'_>, string_type: StringType) -> bool {
    // &, < and > are illegal (as well as " or ' in attributes).
    match iter.st.c {
        '&' | '<' | '>' => true,
        '"' => string_type == StringType::AttributeDouble,
        '\'' => string_type == StringType::AttributeSingle,
        _ => false,
    }
}

fn end_char(string_type: StringType) -> char {
    match string_type {
        StringType::Element => '<',
        StringType::AttributeDouble => '"',
        StringType::AttributeSingle => '\'',
    }
}

fn is_end_char(iter: &Iter<'_>, string_type: StringType) -> bool {
    iter.is(end_char(string_type))
}

fn parse_escape(iter: &mut Iter<'_>) -> Result<char> {
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

fn parse_amp_or_apos(iter: &mut Iter<'_>) -> Result<char> {
    iter.advance_or_die()?;
    if iter.is('m') {
        parse_amp(iter)
    } else if iter.is('p') {
        parse_apos(iter)
    } else {
        parse_err!(iter, "expected either &amp; or &apos;")
    }
}

fn parse_amp(iter: &mut Iter<'_>) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 'p')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('&')
}

fn parse_apos(iter: &mut Iter<'_>) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 'o')?;
    iter.advance_or_die()?;
    expect!(iter, 's')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('\'')
}

fn parse_gt(iter: &mut Iter<'_>) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 't')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('>')
}

fn parse_lt(iter: &mut Iter<'_>) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 't')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('<')
}

fn parse_quot(iter: &mut Iter<'_>) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 'u')?;
    iter.advance_or_die()?;
    expect!(iter, 'o')?;
    iter.advance_or_die()?;
    expect!(iter, 't')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('"')
}

fn parse_codepoint(iter: &mut Iter<'_>) -> Result<char> {
    if iter.peek_is('x') {
        parse_hexidecimal_codepoint(iter)
    } else {
        parse_decimal_codepoint(iter)
    }
}

fn parse_hexidecimal_codepoint(iter: &mut Iter<'_>) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 'x')?;
    iter.advance_or_die()?;
    let mut data = String::new();
    while !iter.is(';') {
        if !iter.is_hex() {
            return parse_err!(iter, "non-hex-digit in hexidecimal unicode escape");
        }
        data.push(iter.st.c);
        iter.advance_or_die()?;
    }
    let codepoint = wrap!(u32::from_str_radix(data.as_str(), 16))?;
    let maybe_char = std::char::from_u32(codepoint);
    match maybe_char {
        Some(c) => Ok(c),
        None => parse_err!(iter, "illegal unicode codepoint '{}'", codepoint),
    }
}

fn parse_decimal_codepoint(iter: &mut Iter<'_>) -> Result<char> {
    iter.advance_or_die()?;
    let mut data = String::new();
    while !iter.is(';') {
        if !iter.is_digit() {
            return parse_err!(iter, "non-digit in decimal unicode escape");
        }
        data.push(iter.st.c);
        iter.advance_or_die()?;
    }
    let codepoint = wrap!(data.parse::<u32>())?;
    let maybe_char = std::char::from_u32(codepoint);
    match maybe_char {
        Some(c) => Ok(c),
        None => parse_err!(iter, "illegal unicode codepoint '{}'", codepoint),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// tests
////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_parse_amp_ok() {
    use crate::parser::Iter;
    let mut iter = Iter::new("mp;").unwrap();
    let c = parse_amp(&mut iter).unwrap();
    assert_eq!('&', c);
    assert_eq!(';', iter.st.c);
}

#[test]
fn test_parse_amp_err() {
    use crate::parser::Iter;
    let mut iter = Iter::new("mp").unwrap();
    let result = parse_amp(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_apos_ok() {
    use crate::parser::Iter;
    let mut iter = Iter::new("pos;").unwrap();
    let c = parse_apos(&mut iter).unwrap();
    assert_eq!('\'', c);
    assert_eq!(';', iter.st.c);
}

#[test]
fn test_parse_apos_err() {
    use crate::parser::Iter;
    let mut iter = Iter::new("pox;").unwrap();
    let result = parse_apos(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_gt_ok() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_t;;").unwrap();
    let c = parse_gt(&mut iter).unwrap();
    assert_eq!('>', c);
    assert_eq!(';', iter.st.c);
}

#[test]
fn test_parse_gt_err() {
    use crate::parser::Iter;
    let mut iter = Iter::new("gt:").unwrap();
    let result = parse_gt(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_lt_ok() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_t;").unwrap();
    let c = parse_lt(&mut iter).unwrap();
    assert_eq!('<', c);
    assert_eq!(';', iter.st.c);
}

#[test]
fn test_parse_lt_err() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_t:").unwrap();
    let result = parse_lt(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_quot_ok() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_uot;").unwrap();
    let c = parse_quot(&mut iter).unwrap();
    assert_eq!('"', c);
    assert_eq!(';', iter.st.c);
}

#[test]
fn test_parse_quot_err() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_uot:").unwrap();
    let result = parse_quot(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_decimal_codepoint_beer_ok() {
    // &#127866; -> 🍺
    use crate::parser::Iter;
    let mut iter = Iter::new("_127866;").unwrap();
    let c = parse_decimal_codepoint(&mut iter).unwrap();
    assert_eq!('🍺', c);
    assert_eq!(';', iter.st.c);
}

#[test]
fn test_parse_decimal_codepoint_beer_err() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_101 52;").unwrap();
    let result = parse_decimal_codepoint(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_hexidecimal_codepoint_beer_ok() {
    // &#x1F37A; -> 🍺
    use crate::parser::Iter;
    let mut iter = Iter::new("_x1F37A;").unwrap();
    let c = parse_hexidecimal_codepoint(&mut iter).unwrap();
    assert_eq!('🍺', c);
    assert_eq!(';', iter.st.c);
}

#[test]
fn test_parse_hexidecimal_codepoint_beer_err() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_x1F37Z;").unwrap();
    let result = parse_hexidecimal_codepoint(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_codepoint_hex_ok() {
    // &#128153; &#x1f499; 💙
    use crate::parser::Iter;
    let mut iter = Iter::new("_x1f499;").unwrap();
    let c = parse_codepoint(&mut iter).unwrap();
    assert_eq!('💙', c);
}

#[test]
fn test_parse_codepoint_decimal_ok() {
    // &#128153; &#x1f499; 💙
    use crate::parser::Iter;
    let mut iter = Iter::new("_128153;").unwrap();
    let c = parse_codepoint(&mut iter).unwrap();
    assert_eq!('💙', c);
}

#[test]
fn test_parse_codepoint_bad_decimal() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_4294967295;").unwrap();
    let result = parse_codepoint(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_codepoint_bad_hexidecimal() {
    use crate::parser::Iter;
    let mut iter = Iter::new("_xFFFFFFFF;").unwrap();
    let result = parse_codepoint(&mut iter);
    assert!(result.is_err());
}

#[test]
fn test_parse_string_ok() {
    let input = "a&amp;b&apos;c&gt;d&lt;e&quot;f&#x1f499;g&#127866;h\"blahblah";
    let expected = "a&b'c>d<e\"f💙g🍺h";
    use crate::parser::Iter;
    let mut iter = Iter::new(input).unwrap();
    let actual = parse_string(&mut iter, StringType::AttributeDouble).unwrap();
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_string_end_err() {
    let input = "a&amp;b&apos;c&gt;d&lt;e&quot;f&#x1f499;g&#127866;h";
    use crate::parser::Iter;
    let mut iter = Iter::new(input).unwrap();
    let result = parse_string(&mut iter, StringType::AttributeDouble);
    assert!(result.is_err());
}

#[test]
fn test_parse_string_bad_escape_err() {
    let input = "a&zoo;\"";
    use crate::parser::Iter;
    let mut iter = Iter::new(input).unwrap();
    let result = parse_string(&mut iter, StringType::AttributeDouble);
    assert!(result.is_err());
}

#[test]
fn test_parse_string_bad_amp_or_apos_err() {
    let input = "a&anp;\"";
    use crate::parser::Iter;
    let mut iter = Iter::new(input).unwrap();
    let result = parse_string(&mut iter, StringType::AttributeDouble);
    assert!(result.is_err());
}

#[test]
fn test_parse_string_quotes_inside_1() {
    let want = r#"attr val with something 'quoted' inside"#;
    let terminated = format!("{}\"", want);
    use crate::parser::Iter;
    let mut iter = Iter::new(terminated.as_str()).unwrap();
    let got = parse_string(&mut iter, StringType::AttributeDouble).unwrap();
    assert_eq!(got, want);
}

#[test]
fn test_parse_string_quotes_inside_2() {
    let want = r#"attr val with something "quoted" inside"#;
    let terminated = format!("{}'", want);
    use crate::parser::Iter;
    let mut iter = Iter::new(terminated.as_str()).unwrap();
    let got = parse_string(&mut iter, StringType::AttributeSingle).unwrap();
    assert_eq!(got, want);
}
