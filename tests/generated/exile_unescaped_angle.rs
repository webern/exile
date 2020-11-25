// generated file, do not edit

use crate::test_utils::run_not_well_formed_test;
use exile::parser::XmlSite;

const INPUT_FILE: &str = "exile_unescaped_angle.xml";

#[test]
/// unescaped angle bracket inside element text
fn unescaped_angle_test() {
    run_not_well_formed_test(
        INPUT_FILE,
        Some(XmlSite {
            line: 4,
            column: 39,
            position: 95,
            character: ' ',
        }),
    );
}
