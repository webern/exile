use xdoc::{ElementData, Node, OrdMap};

use crate::error::Result;
use crate::parser::chars::{is_name_char, is_name_start_char};
use crate::parser::string::{parse_string, StringType};
use crate::parser::{parse_name, skip_comment, skip_processing_instruction, Iter, ParserState};

pub(crate) fn parse_element(iter: &mut Iter) -> Result<ElementData> {
    expect!(iter, '<')?;
    iter.advance_or_die()?;
    let name = parse_name(iter)?;
    let mut element = make_named_element(name.as_str())?;

    // absorb whitespace
    iter.skip_whitespace()?;

    // check and return early if it is an empty, self-closing tag
    if iter.is('/') {
        //println!("It is a self-closing tag with no attributes, i.e. an 'empty' element.");
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
        //println!("It is a self-closing tag with no attributes, i.e. an 'empty' element.");
        iter.advance_or_die()?;
        expect!(iter, '>')?;
        return Ok(element);
    }

    // now the only valid char is '>' and we reach the child nodes
    expect!(iter, '>')?;
    iter.advance_or_die()?;
    parse_children(iter, &mut element)?;
    // TODO - what validation should be done here? why is the iter being advanced?
    while iter.advance() {}
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

fn make_named_element(input: &str) -> Result<ElementData> {
    let split = split_element_name(input)?;
    Ok(ElementData {
        namespace: match split.0 {
            "" => None,
            _ => Some(split.0.to_owned()),
        },
        name: split.1.to_string(),
        attributes: Default::default(),
        nodes: vec![],
    })
}

fn parse_attributes(iter: &mut Iter) -> Result<OrdMap> {
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
        expect!(iter, '"')?;
        iter.advance_or_die()?;
        let value = parse_attribute_value(iter)?;
        expect!(iter, '"')?;
        attributes.mut_map().insert(key, value);
        if !iter.advance() {
            break;
        }
    }
    Ok(attributes)
}

fn parse_attribute_value(iter: &mut Iter) -> Result<String> {
    parse_string(iter, StringType::Attribute)
}

enum LTParse {
    /// The parsed entity was an EndTag.
    EndTag,
    /// The parsed entity was an unsupported node type, i.e. something we want to skip.
    Skip,
    /// The parsed entity was a supported node type.
    Some(Node),
}

fn parse_children(iter: &mut Iter, parent: &mut ElementData) -> Result<()> {
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
            parent.nodes.push(Node::String(text));
        }
        if !iter.is('<') {
            if !iter.advance() {
                break;
            }
        }
    }
    Ok(())
}

fn parse_lt(iter: &mut Iter, parent: &mut ElementData) -> Result<LTParse> {
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
            skip_processing_instruction(iter)?;
            Ok(LTParse::Skip)
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

fn parse_end_tag_name(iter: &mut Iter, parent: &ElementData) -> Result<()> {
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

fn parse_text(iter: &mut Iter) -> Result<String> {
    parse_string(iter, StringType::Element)
}
