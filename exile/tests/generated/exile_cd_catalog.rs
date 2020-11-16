// generated file, do not edit

use crate::test_utils::{path, run_output_test};
use exile::Document;
use xdoc::Declaration;
use xdoc::Encoding;
use xdoc::Version;

const INPUT_FILE: &str = "exile_cd_catalog.xml";
const OUTPUT_FILE: &str = "exile_cd_catalog.output.xml";

#[test]
/// cd_catalog example from https://www.w3schools.com/xml/xml_examples.asp
fn cd_catalog_parse() {
    let path = path(INPUT_FILE);
    let actual = exile::load(&path).unwrap();
    let expected = expected();
    if actual != expected {
        let actual_str = actual.to_string();
        let expected_str = expected.to_string();
        if actual_str != expected_str {
            assert_eq!(expected_str, actual_str);
        } else {
            assert_eq!(expected, actual);
        }
    }
}

#[test]
/// Check that the serialization of this XML document matches what we expect.
fn cd_catalog_serialize() {
    run_output_test(OUTPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: Some(Encoding::Utf8),
    });
    let root = doc.root_mut();
    root.set_name(r#"CATALOG"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"CD"#);
    let gen2n1 = gen1n1.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Empire Burlesque"#);
    let gen2n3 = gen1n1.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Bob Dylan"#);
    let gen2n5 = gen1n1.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"USA"#);
    let gen2n7 = gen1n1.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Columbia"#);
    let gen2n9 = gen1n1.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"10.90"#);
    let gen2n11 = gen1n1.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1985"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"CD"#);
    let gen2n1 = gen1n3.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Hide your heart"#);
    let gen2n3 = gen1n3.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Bonnie Tyler"#);
    let gen2n5 = gen1n3.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n3.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"CBS Records"#);
    let gen2n9 = gen1n3.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"9.90"#);
    let gen2n11 = gen1n3.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1988"#);
    let gen1n5 = root.add_new_child().unwrap();
    gen1n5.set_name(r#"CD"#);
    let gen2n1 = gen1n5.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Greatest Hits"#);
    let gen2n3 = gen1n5.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Dolly Parton"#);
    let gen2n5 = gen1n5.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"USA"#);
    let gen2n7 = gen1n5.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"RCA"#);
    let gen2n9 = gen1n5.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"9.90"#);
    let gen2n11 = gen1n5.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1982"#);
    let gen1n7 = root.add_new_child().unwrap();
    gen1n7.set_name(r#"CD"#);
    let gen2n1 = gen1n7.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Still got the blues"#);
    let gen2n3 = gen1n7.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Gary Moore"#);
    let gen2n5 = gen1n7.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n7.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Virgin records"#);
    let gen2n9 = gen1n7.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"10.20"#);
    let gen2n11 = gen1n7.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1990"#);
    let gen1n9 = root.add_new_child().unwrap();
    gen1n9.set_name(r#"CD"#);
    let gen2n1 = gen1n9.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Eros"#);
    let gen2n3 = gen1n9.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Eros Ramazzotti"#);
    let gen2n5 = gen1n9.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"EU"#);
    let gen2n7 = gen1n9.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"BMG"#);
    let gen2n9 = gen1n9.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"9.90"#);
    let gen2n11 = gen1n9.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1997"#);
    let gen1n11 = root.add_new_child().unwrap();
    gen1n11.set_name(r#"CD"#);
    let gen2n1 = gen1n11.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"One night only"#);
    let gen2n3 = gen1n11.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Bee Gees"#);
    let gen2n5 = gen1n11.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n11.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Polydor"#);
    let gen2n9 = gen1n11.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"10.90"#);
    let gen2n11 = gen1n11.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1998"#);
    let gen1n13 = root.add_new_child().unwrap();
    gen1n13.set_name(r#"CD"#);
    let gen2n1 = gen1n13.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Sylvias Mother"#);
    let gen2n3 = gen1n13.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Dr.Hook"#);
    let gen2n5 = gen1n13.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n13.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"CBS"#);
    let gen2n9 = gen1n13.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.10"#);
    let gen2n11 = gen1n13.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1973"#);
    let gen1n15 = root.add_new_child().unwrap();
    gen1n15.set_name(r#"CD"#);
    let gen2n1 = gen1n15.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Maggie May"#);
    let gen2n3 = gen1n15.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Rod Stewart"#);
    let gen2n5 = gen1n15.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n15.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Pickwick"#);
    let gen2n9 = gen1n15.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.50"#);
    let gen2n11 = gen1n15.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1990"#);
    let gen1n17 = root.add_new_child().unwrap();
    gen1n17.set_name(r#"CD"#);
    let gen2n1 = gen1n17.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Romanza"#);
    let gen2n3 = gen1n17.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Andrea Bocelli"#);
    let gen2n5 = gen1n17.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"EU"#);
    let gen2n7 = gen1n17.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Polydor"#);
    let gen2n9 = gen1n17.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"10.80"#);
    let gen2n11 = gen1n17.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1996"#);
    let gen1n19 = root.add_new_child().unwrap();
    gen1n19.set_name(r#"CD"#);
    let gen2n1 = gen1n19.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"When a man loves a woman"#);
    let gen2n3 = gen1n19.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Percy Sledge"#);
    let gen2n5 = gen1n19.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"USA"#);
    let gen2n7 = gen1n19.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Atlantic"#);
    let gen2n9 = gen1n19.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.70"#);
    let gen2n11 = gen1n19.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1987"#);
    let gen1n21 = root.add_new_child().unwrap();
    gen1n21.set_name(r#"CD"#);
    let gen2n1 = gen1n21.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Black angel"#);
    let gen2n3 = gen1n21.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Savage Rose"#);
    let gen2n5 = gen1n21.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"EU"#);
    let gen2n7 = gen1n21.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Mega"#);
    let gen2n9 = gen1n21.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"10.90"#);
    let gen2n11 = gen1n21.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1995"#);
    let gen1n23 = root.add_new_child().unwrap();
    gen1n23.set_name(r#"CD"#);
    let gen2n1 = gen1n23.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"1999 Grammy Nominees"#);
    let gen2n3 = gen1n23.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Many"#);
    let gen2n5 = gen1n23.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"USA"#);
    let gen2n7 = gen1n23.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Grammy"#);
    let gen2n9 = gen1n23.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"10.20"#);
    let gen2n11 = gen1n23.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1999"#);
    let gen1n25 = root.add_new_child().unwrap();
    gen1n25.set_name(r#"CD"#);
    let gen2n1 = gen1n25.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"For the good times"#);
    let gen2n3 = gen1n25.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Kenny Rogers"#);
    let gen2n5 = gen1n25.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n25.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Mucik Master"#);
    let gen2n9 = gen1n25.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.70"#);
    let gen2n11 = gen1n25.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1995"#);
    let gen1n27 = root.add_new_child().unwrap();
    gen1n27.set_name(r#"CD"#);
    let gen2n1 = gen1n27.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Big Willie style"#);
    let gen2n3 = gen1n27.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Will Smith"#);
    let gen2n5 = gen1n27.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"USA"#);
    let gen2n7 = gen1n27.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Columbia"#);
    let gen2n9 = gen1n27.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"9.90"#);
    let gen2n11 = gen1n27.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1997"#);
    let gen1n29 = root.add_new_child().unwrap();
    gen1n29.set_name(r#"CD"#);
    let gen2n1 = gen1n29.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Tupelo Honey"#);
    let gen2n3 = gen1n29.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Van Morrison"#);
    let gen2n5 = gen1n29.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n29.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Polydor"#);
    let gen2n9 = gen1n29.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.20"#);
    let gen2n11 = gen1n29.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1971"#);
    let gen1n31 = root.add_new_child().unwrap();
    gen1n31.set_name(r#"CD"#);
    let gen2n1 = gen1n31.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Soulsville"#);
    let gen2n3 = gen1n31.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Jorn Hoel"#);
    let gen2n5 = gen1n31.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"Norway"#);
    let gen2n7 = gen1n31.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"WEA"#);
    let gen2n9 = gen1n31.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"7.90"#);
    let gen2n11 = gen1n31.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1996"#);
    let gen1n33 = root.add_new_child().unwrap();
    gen1n33.set_name(r#"CD"#);
    let gen2n1 = gen1n33.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"The very best of"#);
    let gen2n3 = gen1n33.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Cat Stevens"#);
    let gen2n5 = gen1n33.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n33.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Island"#);
    let gen2n9 = gen1n33.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.90"#);
    let gen2n11 = gen1n33.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1990"#);
    let gen1n35 = root.add_new_child().unwrap();
    gen1n35.set_name(r#"CD"#);
    let gen2n1 = gen1n35.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Stop"#);
    let gen2n3 = gen1n35.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Sam Brown"#);
    let gen2n5 = gen1n35.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n35.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"A and M"#);
    let gen2n9 = gen1n35.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.90"#);
    let gen2n11 = gen1n35.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1988"#);
    let gen1n37 = root.add_new_child().unwrap();
    gen1n37.set_name(r#"CD"#);
    let gen2n1 = gen1n37.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Bridge of Spies"#);
    let gen2n3 = gen1n37.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"T'Pau"#);
    let gen2n5 = gen1n37.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n37.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Siren"#);
    let gen2n9 = gen1n37.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"7.90"#);
    let gen2n11 = gen1n37.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1987"#);
    let gen1n39 = root.add_new_child().unwrap();
    gen1n39.set_name(r#"CD"#);
    let gen2n1 = gen1n39.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Private Dancer"#);
    let gen2n3 = gen1n39.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Tina Turner"#);
    let gen2n5 = gen1n39.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n39.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Capitol"#);
    let gen2n9 = gen1n39.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.90"#);
    let gen2n11 = gen1n39.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1983"#);
    let gen1n41 = root.add_new_child().unwrap();
    gen1n41.set_name(r#"CD"#);
    let gen2n1 = gen1n41.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Midt om natten"#);
    let gen2n3 = gen1n41.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Kim Larsen"#);
    let gen2n5 = gen1n41.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"EU"#);
    let gen2n7 = gen1n41.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Medley"#);
    let gen2n9 = gen1n41.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"7.80"#);
    let gen2n11 = gen1n41.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1983"#);
    let gen1n43 = root.add_new_child().unwrap();
    gen1n43.set_name(r#"CD"#);
    let gen2n1 = gen1n43.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Pavarotti Gala Concert"#);
    let gen2n3 = gen1n43.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Luciano Pavarotti"#);
    let gen2n5 = gen1n43.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n43.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"DECCA"#);
    let gen2n9 = gen1n43.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"9.90"#);
    let gen2n11 = gen1n43.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1991"#);
    let gen1n45 = root.add_new_child().unwrap();
    gen1n45.set_name(r#"CD"#);
    let gen2n1 = gen1n45.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"The dock of the bay"#);
    let gen2n3 = gen1n45.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Otis Redding"#);
    let gen2n5 = gen1n45.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"USA"#);
    let gen2n7 = gen1n45.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Stax Records"#);
    let gen2n9 = gen1n45.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"7.90"#);
    let gen2n11 = gen1n45.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1968"#);
    let gen1n47 = root.add_new_child().unwrap();
    gen1n47.set_name(r#"CD"#);
    let gen2n1 = gen1n47.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Picture book"#);
    let gen2n3 = gen1n47.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Simply Red"#);
    let gen2n5 = gen1n47.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"EU"#);
    let gen2n7 = gen1n47.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"Elektra"#);
    let gen2n9 = gen1n47.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"7.20"#);
    let gen2n11 = gen1n47.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1985"#);
    let gen1n49 = root.add_new_child().unwrap();
    gen1n49.set_name(r#"CD"#);
    let gen2n1 = gen1n49.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Red"#);
    let gen2n3 = gen1n49.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"The Communards"#);
    let gen2n5 = gen1n49.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"UK"#);
    let gen2n7 = gen1n49.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"London"#);
    let gen2n9 = gen1n49.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"7.80"#);
    let gen2n11 = gen1n49.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1987"#);
    let gen1n51 = root.add_new_child().unwrap();
    gen1n51.set_name(r#"CD"#);
    let gen2n1 = gen1n51.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    gen2n1.add_text(r#"Unchain my heart"#);
    let gen2n3 = gen1n51.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    gen2n3.add_text(r#"Joe Cocker"#);
    let gen2n5 = gen1n51.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    gen2n5.add_text(r#"USA"#);
    let gen2n7 = gen1n51.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    gen2n7.add_text(r#"EMI"#);
    let gen2n9 = gen1n51.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    gen2n9.add_text(r#"8.20"#);
    let gen2n11 = gen1n51.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    gen2n11.add_text(r#"1987"#);
    doc
}
