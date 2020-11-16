// generated file, do not edit

use crate::test_utils::run_not_well_formed_test;
use exile::error::XmlSite;

const INPUT_FILE: &str = "exile_angle_in_attribute_value.xml";

#[test]
/// unescaped angle bracket in an attribute value
fn angle_in_attribute_value_test() {
    run_not_well_formed_test(
        INPUT_FILE,
        Some(XmlSite {
            line: 2,
            column: 12,
            position: 51,
            character: '<',
        }),
    );
}
