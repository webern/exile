use crate::error::Result;
use crate::parser::chars::is_name_start_char;
use crate::parser::pi::parse_pi;
use crate::parser::string::{parse_string, StringType};
use crate::parser::{parse_name, skip_comment, Iter};
use crate::xdoc::OrdMap;
use crate::{Element, Misc, Node};

pub(crate) fn parse_element(iter: &mut Iter<'_>) -> Result<Element> {
    expect!(iter, '<')?;
    iter.advance_or_die()?;
    let name = parse_name(iter)?;
    let mut element = make_named_element(name.as_str())?;

    // absorb whitespace
    iter.skip_whitespace()?;

    // check and return early if it is an empty, self-closing tag
    if iter.is('/') {
        iter.advance_or_die()?;
        expect!(iter, '>')?;
        return Ok(element);
    }

    // now the only valid chars are '>' or the start of an attribute name
    if iter.is_name_start_char() {
        element.attributes = parse_attributes(iter)?;
    }

    // check and return early if it is an empty, self-closing tag that had attributes
    if iter.is('/') {
        iter.advance_or_die()?;
        expect!(iter, '>')?;
        return Ok(element);
    }

    // now the only valid char is '>' and we reach the child nodes
    expect!(iter, '>')?;
    iter.advance_or_die()?;
    parse_children(iter, &mut element)?;
    // TODO - what validation should be done here? why is the iter being advanced?
    // while iter.advance() {}
    expect!(iter, '>')?;
    Ok(element)
}

fn split_element_name(input: &str) -> Result<(&str, &str)> {
    let split: Vec<&str> = input.split(':').collect();
    match split.len() {
        1 => Ok(("", split.first().unwrap())),
        2 => Ok((split.first().unwrap(), split.last().unwrap())),
        _ => raise!(""),
    }
}

fn make_named_element(input: &str) -> Result<Element> {
    let split = split_element_name(input)?;
    Ok(Element {
        namespace: match split.0 {
            "" => None,
            _ => Some(split.0.to_owned()),
        },
        name: split.1.to_string(),
        attributes: Default::default(),
        nodes: vec![],
    })
}

fn parse_attributes(iter: &mut Iter<'_>) -> Result<OrdMap> {
    let mut attributes = OrdMap::new();
    loop {
        iter.skip_whitespace()?;
        if iter.is('/') || iter.is('>') {
            break;
        }
        let key = if iter.is_name_start_char() {
            parse_name(iter)?
        } else {
            String::default()
        };
        iter.skip_whitespace()?;
        expect!(iter, '=')?;
        iter.advance_or_die()?;
        iter.skip_whitespace()?;
        let (start, string_type) = attribute_start_quote(iter)?;
        iter.advance_or_die()?;
        let value = parse_attribute_value(iter, string_type)?;
        expect!(iter, start)?;
        attributes.mut_map().insert(key, value);
        if !iter.advance() {
            break;
        }
    }
    Ok(attributes)
}

fn attribute_start_quote(iter: &Iter<'_>) -> Result<(char, StringType)> {
    let c = iter.st.c;
    match c {
        '\'' => Ok((c, StringType::AttributeSingle)),
        '"' => Ok((c, StringType::AttributeDouble)),
        _ => raise!(
            "expected attribute value to start with either a single or double quote, got '{}'",
            c
        ),
    }
}

/// Expects the iter to be pointing at the first character of the string.
fn parse_attribute_value(iter: &mut Iter<'_>, string_type: StringType) -> Result<String> {
    match string_type {
        StringType::AttributeDouble | StringType::AttributeSingle => {
            parse_string(iter, string_type)
        }
        _ => raise!(
            "bug: the wrong function was called for a string of type: {:?}",
            string_type
        ),
    }
}

// this function takes over after an element's opening tag (the parent element) has been parsed.
// the nodes that are contained by the parent are parsed and added to the parent. this function is
// recursive descending until an element with no children is reached.
fn parse_children(iter: &mut Iter<'_>, parent: &mut Element) -> Result<()> {
    loop {
        iter.skip_whitespace()?;
        if iter.is('<') {
            let lt_parse = parse_lt(iter, parent)?;
            match lt_parse {
                LTParse::EndTag => {
                    // this is the recursion's breaking condition
                    return Ok(());
                }
                LTParse::Skip => {
                    // do nothing
                }
                LTParse::Some(node) => {
                    parent.nodes.push(node);
                }
            }
        } else {
            let text = parse_text(iter)?;
            parent.nodes.push(Node::Text(text));
        }
        // some parsing functions may return with the iter pointing to the last thing that was part
        // of their construct, while others might advance the iter to the next char *after* the
        // thing they parsed. to deal with this we check, are we pointing to a '<' currently? if so,
        // we do not want to advance the iter. also, if we have reached the end, we want to break
        // out of the loop.
        if !iter.is('<') && !iter.advance() {
            break;
        }
    }
    Ok(())
}

// the return type for `parse_lt`. since the caller of `parse_lt` doesn't know what type of node
// has been encountered, this enum is used to describe what was parsed.
enum LTParse {
    // the parsed entity was an EndTag.
    EndTag,
    // the parsed entity was an unsupported node type, i.e. something we want to skip.
    Skip,
    // the parsed entity was a supported node type.
    Some(Node),
}

// parse the correct type of node (or end tag) when encountering a '<'
fn parse_lt(iter: &mut Iter<'_>, parent: &mut Element) -> Result<LTParse> {
    let next = iter.peek_or_die()?;
    // do the most common, hottest path first
    if is_name_start_char(next) {
        let element = parse_element(iter)?;
        return Ok(LTParse::Some(Node::Element(element)));
    }
    match next {
        '/' => {
            parse_end_tag_name(iter, parent)?;
            Ok(LTParse::EndTag)
        }
        '?' => {
            let pi = parse_pi(iter)?;
            Ok(LTParse::Some(Node::Misc(Misc::PI(pi))))
        }
        '!' => {
            // skip comment expects the iter to be advanced passed lt
            iter.advance_or_die()?;
            skip_comment(iter)?;
            Ok(LTParse::Skip)
        }
        _ => {
            // this error occurred on the peeked char, so to report the correct position of the
            // error, we will first advance the iter (if possible).
            iter.advance();
            parse_err!(iter, "unexpected char following '<'")
        }
    }
}

// takes an iter pointing at '<' where the next character is required to be '/'. parses the name of
// the end tag and compares it to make sure it matches `parent`. if anything goes wrong, Err.
fn parse_end_tag_name(iter: &mut Iter<'_>, parent: &Element) -> Result<()> {
    expect!(iter, '<')?;
    iter.advance_or_die()?;
    expect!(iter, '/')?;
    iter.advance_or_die()?;
    iter.skip_whitespace()?;
    iter.expect_name_start_char()?;
    let mut name = String::default();
    name.push(iter.st.c);
    loop {
        iter.advance_or_die()?;
        if iter.is('>') || iter.is_whitespace() {
            break;
        } else if iter.is_name_char() {
            name.push(iter.st.c);
        } else {
            return parse_err!(iter);
        }
    }
    iter.skip_whitespace()?;
    expect!(iter, '>')?;
    if name != parent.fullname() {
        return parse_err!(
            iter,
            "closing element name '{}' does not match openeing element name '{}'",
            name,
            parent.fullname()
        );
    }
    Ok(())
}

fn parse_text(iter: &mut Iter<'_>) -> Result<String> {
    parse_string(iter, StringType::Element)
}

#[test]
fn parse_attribute_value_test_1() {
    let mut iter = Iter::new(r#"some "fun" attribute value'"#).unwrap();
    let value = parse_attribute_value(&mut iter, StringType::AttributeSingle).unwrap();
    assert_eq!(value, r#"some "fun" attribute value"#);
    assert_eq!(iter.st.c, '\'');
}