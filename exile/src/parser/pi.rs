use xdoc::{is_whitespace, PI};

use crate::error::Result;
use crate::parser::Iter;

use super::chars::{is_name_char, is_name_start_char};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum PIStatus {
    BeforeTarget,
    InsideTarget,
    AfterTarget,
    AfterInstruction,
    QuestionMark,
    Close,
}

impl Default for PIStatus {
    fn default() -> Self {
        PIStatus::BeforeTarget
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
struct PIProcessor {
    status: PIStatus,
    target: String,
    instructions: Vec<String>,
}

/// The iter should be pointing to the opening `<` of a processing instruction.
pub(crate) fn parse_pi_logic(iter: &mut Iter<'_>) -> Result<(String, Vec<String>)> {
    expect!(iter, '<')?;
    iter.advance_or_die()?;
    expect!(iter, '?')?;
    iter.advance_or_die()?;
    let mut processor = PIProcessor::default();
    loop {
        take_processing_instruction_char(iter, &mut processor)?;
        if processor.status == PIStatus::Close {
            break;
        }
        iter.advance_or_die()?;
    }
    Ok((processor.target, processor.instructions))
}

pub(crate) fn parse_pi(iter: &mut Iter<'_>) -> Result<PI> {
    let (target, instructions) = parse_pi_logic(iter)?;
    Ok(PI {
        target,
        instructions,
    })
}

fn take_processing_instruction_char(
    iter: &mut Iter<'_>,
    processor: &mut PIProcessor,
) -> Result<()> {
    match processor.status {
        PIStatus::BeforeTarget => {
            if !is_name_start_char(iter.st.c) {
                return parse_err!(iter);
            } else {
                processor.target.push(iter.st.c);
                processor.status = PIStatus::InsideTarget;
            }
        }
        PIStatus::InsideTarget => {
            if iter.st.c.is_ascii_whitespace() {
                processor.status = PIStatus::AfterTarget;
            } else if !is_name_char(iter.st.c) {
                return parse_err!(iter);
            } else {
                processor.target.push(iter.st.c);
            }
        }
        PIStatus::AfterTarget | PIStatus::AfterInstruction => {
            if iter.st.c == '?' {
                processor.status = PIStatus::QuestionMark;
            } else if !iter.is_whitespace() {
                let instruction = parse_pi_string(iter)?;
                processor.instructions.push(instruction);
                if iter.is('?') {
                    processor.status = PIStatus::QuestionMark;
                } else if !iter.is_whitespace() {
                    return parse_err!(iter);
                } else {
                    processor.status = PIStatus::AfterInstruction;
                }
            }
        }
        PIStatus::QuestionMark => {
            if iter.st.c == '>' {
                processor.status = PIStatus::Close;
            } else {
                return parse_err!(iter);
            }
        }
        PIStatus::Close => { /* done */ }
    }
    Ok(())
}

fn parse_pi_string(iter: &mut Iter<'_>) -> Result<String> {
    let mut buf = String::new();
    loop {
        if iter.is_whitespace() {
            return Ok(buf);
        } else if iter.is('?') && iter.peek_or_die()? == '>' {
            return Ok(buf);
        } else {
            buf.push(iter.st.c);
        }
        if !iter.advance() {
            break;
        }
    }
    Ok(buf)
}

#[test]
fn parse_pi_string_test() {
    struct TestCase {
        input: &'static str,
        want: &'static str,
        iter: char,
    }
    let test_cases = vec![
        TestCase {
            input: "bloop bleep",
            want: "bloop",
            iter: ' ',
        },
        TestCase {
            input: "bloop?bleep",
            want: "bloop?bleep",
            iter: 'p',
        },
        TestCase {
            input: "bloop?>bleep",
            want: "bloop",
            iter: '?',
        },
        TestCase {
            input: "beer🍺🍺🍺 🍺🍺?>",
            want: "beer🍺🍺🍺",
            iter: ' ',
        },
        TestCase {
            input: "beer🍺🍺🍺🍺🍺",
            want: "beer🍺🍺🍺🍺🍺",
            iter: '🍺',
        },
    ];
    for test_case in &test_cases {
        let mut iter = Iter::new(test_case.input).unwrap();
        let got = parse_pi_string(&mut iter).unwrap();
        assert_eq!(
            got.as_str(),
            test_case.want,
            "parse_pi_string(\"{}\") returned '{}', expected '{}'",
            test_case.input,
            got.as_str(),
            test_case.want
        );
        assert_eq!(
            iter.st.c, test_case.iter,
            "expected iter to be pointing at '{}', got '{}'",
            test_case.iter, iter.st.c
        );
    }
}
