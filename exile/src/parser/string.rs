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
    iter.advance_or_die()?;
    if iter.is('m') {
        parse_amp(iter)
    } else if iter.is('p') {
        parse_apos(iter)
    } else {
        parse_err!(iter, "expected either &amp; or &apos;")
    }
}

fn parse_amp(iter: &mut Iter) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 'p')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('&')
}

fn parse_apos(iter: &mut Iter) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 'o')?;
    iter.advance_or_die()?;
    expect!(iter, 's')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('\'')
}

fn parse_gt(iter: &mut Iter) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 't')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('>')
}

fn parse_lt(iter: &mut Iter) -> Result<char> {
    iter.advance_or_die()?;
    expect!(iter, 't')?;
    iter.advance_or_die()?;
    expect!(iter, ';')?;
    Ok('<')
}

fn parse_quot(iter: &mut Iter) -> Result<char> {
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

fn parse_codepoint(iter: &mut Iter) -> Result<char> {
    // TODO - implement
    while iter.advance() {
        if iter.is(';') {
            break;
        }
    }
    Ok('🍺')
}
