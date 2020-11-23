use crate::error::Result;
use crate::parser::Iter;
use crate::PI;

// use super::chars::{is_name_char, is_name_start_char};

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
// enum PIStatus {
//     BeforeTarget,
//     InsideTarget,
//     AfterTarget,
//     AfterInstruction,
//     QuestionMark,
//     Close,
// }

// impl Default for PIStatus {
//     fn default() -> Self {
//         PIStatus::BeforeTarget
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
// struct PIProcessor {
//     status: PIStatus,
//     target: String,
//     instructions: Vec<String>,
// }

/// The iter should be pointing to the opening `<` of a processing instruction.
pub(crate) fn parse_pi_logic(iter: &mut Iter<'_>) -> Result<(String, String)> {
    // expect!(iter, '<')?;
    // iter.advance_or_die()?;
    // expect!(iter, '?')?;
    // iter.advance_or_die()?;
    // let mut processor = PIProcessor::default();
    // loop {
    //     take_processing_instruction_char(iter, &mut processor)?;
    //     if processor.status == PIStatus::Close {
    //         break;
    //     }
    //     iter.advance_or_die()?;
    // }
    Ok(("".into(), "".into()))
}

/// The iter should be pointing to the opening `<` of a processing instruction.
pub(crate) fn parse_pi(iter: &mut Iter<'_>) -> Result<PI> {
    let (target, data) = parse_pi_logic(iter)?;
    Ok(PI { target, data })
}

// fn take_processing_instruction_char(
//     iter: &mut Iter<'_>,
//     processor: &mut PIProcessor,
// ) -> Result<()> {
//     match processor.status {
//         PIStatus::BeforeTarget => {
//             if iter.is('?') {
//                 // this is something funky like <??> or <?? >, which are both valid.
//                 processor.status = PIStatus::QuestionMark;
//             } else if !is_name_start_char(iter.st.c) {
//                 return parse_err!(iter);
//             } else {
//                 processor.target.push(iter.st.c);
//                 processor.status = PIStatus::InsideTarget;
//             }
//         }
//         PIStatus::InsideTarget => {
//             if iter.st.c.is_ascii_whitespace() {
//                 processor.status = PIStatus::AfterTarget;
//             } else if iter.is('?') {
//                 // something like this <?xyz?>
//                 processor.status = PIStatus::QuestionMark;
//             } else if !is_name_char(iter.st.c) {
//                 return parse_err!(iter);
//             } else {
//                 processor.target.push(iter.st.c);
//             }
//         }
//         PIStatus::AfterTarget | PIStatus::AfterInstruction => {
//             if iter.st.c == '?' {
//                 processor.status = PIStatus::QuestionMark;
//             } else if !iter.is_whitespace() {
//                 let instruction = parse_pi_string(iter)?;
//                 processor.instructions.push(instruction);
//                 if iter.is('?') {
//                     processor.status = PIStatus::QuestionMark;
//                 } else if !iter.is_whitespace() {
//                     return parse_err!(iter);
//                 } else {
//                     processor.status = PIStatus::AfterInstruction;
//                 }
//             }
//         }
//         PIStatus::QuestionMark => {
//             iter.skip_whitespace()?;
//             if iter.st.c == '>' {
//                 processor.status = PIStatus::Close;
//             } else {
//                 // a question mark can be a valid instruction. since the question mark wasn't
//                 // followed by > we are not done and consider an instructions.
//
//             }
//         }
//         PIStatus::Close => { /* done */ }
//     }
//     Ok(())
// }
//
// fn is_pi_close(iter: &mut Iter<'_>) -> Result<bool> {
//     Ok(iter.is('?') && iter.peek_or_die()? == '>')
// }
//
// fn parse_pi_string(iter: &mut Iter<'_>) -> Result<String> {
//     let mut buf = String::new();
//     loop {
//         if iter.is_whitespace() || is_pi_close(iter)? {
//             return Ok(buf);
//         } else {
//             buf.push(iter.st.c);
//         }
//         if !iter.advance() {
//             break;
//         }
//     }
//     Ok(buf)
// }

////////////////////////////////////////////////////////////////////////////////////////////////////

// #[test]
// fn parse_pi_string_test() {
//     struct TestCase {
//         input: &'static str,
//         want: &'static str,
//         iter: char,
//     }
//     let test_cases = vec![
//         TestCase {
//             input: "bloop bleep",
//             want: "bloop",
//             iter: ' ',
//         },
//         TestCase {
//             input: "bloop?bleep",
//             want: "bloop?bleep",
//             iter: 'p',
//         },
//         TestCase {
//             input: "bloop?>bleep",
//             want: "bloop",
//             iter: '?',
//         },
//         TestCase {
//             input: "beer🍺🍺🍺 🍺🍺?>",
//             want: "beer🍺🍺🍺",
//             iter: ' ',
//         },
//         TestCase {
//             input: "beer🍺🍺🍺🍺🍺",
//             want: "beer🍺🍺🍺🍺🍺",
//             iter: '🍺',
//         },
//     ];
//     for test_case in &test_cases {
//         let mut iter = Iter::new(test_case.input).unwrap();
//         let got = parse_pi_string(&mut iter).unwrap();
//         assert_eq!(
//             got.as_str(),
//             test_case.want,
//             "parse_pi_string(\"{}\") returned '{}', expected '{}'",
//             test_case.input,
//             got.as_str(),
//             test_case.want
//         );
//         assert_eq!(
//             iter.st.c, test_case.iter,
//             "expected iter to be pointing at '{}', got '{}'",
//             test_case.iter, iter.st.c
//         );
//     }
// }

#[test]
fn parse_pi_funky_1() {
    let pi_str = "<?pi some data ? >";
    let mut iter = Iter::new(pi_str).unwrap();
    let pi = parse_pi(&mut iter).unwrap();
    let parse_result = parse_pi(&mut iter);
    assert!(parse_result.is_err());
}

#[test]
fn parse_pi_funky_2() {
    let pi_str = "<??>";
    let mut iter = Iter::new(pi_str).unwrap();
    let pi = parse_pi(&mut iter).unwrap();
    assert_eq!("", pi.target);
    assert!(pi.data.is_empty());
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
    assert_eq!("bones", pi.target);
    assert!(pi.data.is_empty());
}

#[test]
fn parse_pi_funky_6() {
    // this is from jclark_valid_sa_017.xml
    let pi_str = "<?pi some data ? > <??>";
    let mut iter = Iter::new(pi_str).unwrap();
    let pi = parse_pi(&mut iter).unwrap();
    assert_eq!("pi", pi.target);
    assert_eq!("some data ? > <?", pi.data);
}
