#[test]
fn good_syntax_ezfile_test() {
    let info = xtest::load("ezfile");
    let xml_str = info.read_xml_file();
    let parse_result = exile::parse_str(xml_str.as_str());
    assert!(parse_result.is_ok());
}
