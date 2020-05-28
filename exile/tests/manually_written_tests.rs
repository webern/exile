#[test]
fn good_syntax_ezfile_test() {
    let info = xtest::load("ezfile");
    let xml_str = info.read_xml_file();
    let parse_result = exile::parse(xml_str.as_str());
    assert!(parse_result.is_ok());
}

/*
// This is used for file generation and is checked-in while commented out intentionally.
#[test]
fn genstuff() {
    use std::fs::{read_to_string, OpenOptions};
    use std::io::Cursor;
    use std::path::PathBuf;
    use xtest::{Metadata, Syntax};
    let name = "simple_musicxml";
    let data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("xtest")
        .join("data")
        .canonicalize()
        .unwrap();

    let inpath = data_path.join(format!("{}.xml", name));
    let xmlout_path = data_path.join(format!("{}.expected.xml", name));
    let jsonout_path = data_path.join(format!("{}.metadata.json", name));
    println!("{}", inpath.display());
    let xml = read_to_string(inpath).unwrap();
    let doc = exile::parse(xml.as_str()).unwrap();
    let md = xtest::Metadata {
        description: "cd_catalog example from https://www.w3schools.com/xml/xml_examples.asp"
            .to_string(),
        syntax: xtest::Syntax::Good {},
        expected: Some(doc.clone()),
    };
    let json = serde_json::to_string_pretty(&md).unwrap();
    println!("{}", json);
    let mut cursor = Cursor::new(Vec::new());
    let result = doc.write(&mut cursor);
    assert!(result.is_ok());
    let data = cursor.into_inner();
    {
        let _ = std::fs::remove_file(&xmlout_path);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(xmlout_path)
            .unwrap();
        file.write_all(data.as_slice()).unwrap();
    }
    {
        let _ = std::fs::remove_file(&jsonout_path);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(jsonout_path)
            .unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}
*/
