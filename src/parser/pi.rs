use crate::parser::error::Result;
use crate::parser::Iter;
use crate::Pi;

/// The iter should be pointing to the opening `<` of a processing instruction.
pub(crate) fn parse_pi_logic(iter: &mut Iter<'_>) -> Result<(String, String)> {
    expect!(iter, '<')?;
    iter.advance_or_die()?;
    expect!(iter, '?')?;
    iter.advance_or_die()?;

    // handle the special case <??>
    if iter.is('?') {
        iter.advance_or_die()?;
        expect!(iter, '>')?;
        iter.advance();
        return Ok(("".into(), "".into()));
    }

    let target = parse_pi_target(iter)?;
    let mut data = String::new();
    loop {
        if iter.is('?') && iter.peek_is('>') {
            iter.advance_or_die()?;
            iter.advance();
            break;
        }
        data.push(iter.st.c);
        iter.advance_or_die()?;
    }
    Ok((target, data))
}

/// Must be a valid name terminated by whitespace.
fn parse_pi_target(iter: &mut Iter<'_>) -> Result<String> {
    if !iter.is_name_start_char() {
        return parse_err!(iter, "expected name start char, found '{}'", iter.st.c);
    }
    let mut name = String::new();
    name.push(iter.st.c);
    iter.advance_or_die()?;
    loop {
        if iter.is_whitespace() {
            iter.advance_or_die()?;
            break;
        } else if iter.is('?') {
            // e.g. <?target??
            break;
        } else if !iter.is_name_char() {
            return parse_err!(iter, "expected name char, found '{}'", iter.st.c);
        } else {
            name.push(iter.st.c);
        }
        iter.advance_or_die()?;
    }
    Ok(name)
}

/// The iter should be pointing to the opening `<` of a processing instruction.
pub(crate) fn parse_pi(iter: &mut Iter<'_>) -> Result<Pi> {
    let (target, data) = parse_pi_logic(iter)?;
    Ok(Pi::new_unchecked(target, data))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn parse_pi_easy() {
    let pi_str = "<?target data?>";
    let mut iter = Iter::new(pi_str).unwrap();
    let pi = parse_pi(&mut iter).unwrap();
    assert_eq!("target", pi.target());
    assert_eq!("data", pi.data());
    assert!(!iter.advance());
}

#[test]
fn parse_pi_peasy() {
    let pi_str = "<?target data?>X";
    let mut iter = Iter::new(pi_str).unwrap();
    let pi = parse_pi(&mut iter).unwrap();
    assert_eq!("target", pi.target());
    assert_eq!("data", pi.data());
    assert!(iter.is('X'));
}

#[test]
fn parse_pi_funky_1() {
    let pi_str = "<?pi some data ? >";
    let mut iter = Iter::new(pi_str).unwrap();
    let parse_result = parse_pi(&mut iter);
    assert!(parse_result.is_err());
}

#[test]
fn parse_pi_funky_2() {
    let pi_str = "<??>";
    let mut iter = Iter::new(pi_str).unwrap();
    let pi = parse_pi(&mut iter).unwrap();
    assert_eq!("", pi.target());
    assert!(pi.data().is_empty());
}

#[test]
fn parse_pi_funky_3() {
    // established as not-well-formed by jclark_not_wf_sa_003.xml
    let pi_str = "<? ?>";
    let mut iter = Iter::new(pi_str).unwrap();
    let parse_result = parse_pi(&mut iter);
    assert!(parse_result.is_err());
}

#[test]
fn parse_pi_funky_4() {
    let pi_str = "< ? ? >";
    let mut iter = Iter::new(pi_str).unwrap();
    let parse_result = parse_pi(&mut iter);
    assert!(parse_result.is_err());
}

#[test]
fn parse_pi_funky_5() {
    let pi_str = "<?bones?>";
    let mut iter = Iter::new(pi_str).unwrap();
    let pi = parse_pi(&mut iter).unwrap();
    assert_eq!("bones", pi.target());
    assert!(pi.data().is_empty());
}

#[test]
fn parse_pi_funky_6() {
    // this is from jclark_valid_sa_017.xml
    let pi_str = "<?pi some data ? > <??>";
    let mut iter = Iter::new(pi_str).unwrap();
    let pi = parse_pi(&mut iter).unwrap();
    assert_eq!("pi", pi.target());
    assert_eq!("some data ? > <?", pi.data());
}
