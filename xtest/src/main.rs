use std::path::PathBuf;

use xdoc::{Declaration, Document, Element, Encoding, Node, PI, Version};
use xtest::{Metadata, Syntax};

fn main() {
    let _path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .canonicalize()
        .unwrap();

    let _t = Metadata {
        description: "the things are described here".to_string(),
        syntax: Syntax::Bad {
            character_position: 21,
            line: 2,
            column: 10,
        },
        expected: None,
    };

    let mut doc = Document::new();
    doc.set_declaration(Declaration { version: Version::OneDotOne, encoding: Encoding::Utf8 });
    // TODO - add <?a b?>
    let root = Element {
        namespace: None,
        name: "c".to_owned(),
        attributes: Default::default(),
        nodes: vec![
            Node::PI(PI { target: "d".to_owned(), instructions: vec!["e".to_owned()] }),
            Node::Element(Element {
                namespace: None,
                name: "f".to_owned(),
                attributes: Default::default(),
                nodes: vec![],
            }),
            Node::PI(PI { target: "d".to_owned(), instructions: vec!["e".to_owned()] }),
        ],
    };
    // TODO - add <?i j?>

    doc.set_root(root);

    let the_test = Metadata {
        description: "a simple file with processing instructions".to_owned(),
        syntax: Default::default(),
        expected: Some(doc.clone()),
    };
    println!("{}", serde_json::to_string_pretty(&the_test).unwrap());
    //
    // let x = Metadata {
    //     description: "x".to_string(),
    //     syntax: Syntax::Good {},
    //     expected: Some(xdoc::Document::new()),
    // };
    //
    // println!("{}", serde_json::to_string_pretty(&x).unwrap());
    println!("{}", doc.to_string());
}
