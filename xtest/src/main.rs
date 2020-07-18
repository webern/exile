use std::path::{Path, PathBuf};

use xdoc::{Declaration, Document, Element, Encoding, Misc, Node, PI, Version};
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
    doc.push_prolog_misc(Misc::PI(PI { target: "a".to_owned(), instructions: vec!["b".to_owned()] }));
    let root = Element {
        namespace: None,
        name: "c".to_owned(),
        attributes: Default::default(),
        nodes: vec![
            Node::Misc(Misc::PI(PI { target: "d".to_owned(), instructions: vec!["e".to_owned()] })),
            Node::Element(Element {
                namespace: None,
                name: "f".to_owned(),
                attributes: Default::default(),
                nodes: vec![],
            }),
            Node::Misc(Misc::PI(PI { target: "g".to_owned(), instructions: vec!["h".to_owned()] })),
        ],
    };
    doc.push_epilog_misc(Misc::PI(PI { target: "i".to_owned(), instructions: vec!["j".to_owned()] }));
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

    print_path_and_children(&PathBuf::from("./"));
}

fn print_dir(p: &Path) {
    if !p.is_dir() {
        panic!("{} is not a dir", p.display());
    }
    let paths = std::fs::read_dir(p).unwrap();

    for path in paths {
        print_path_and_children(&path.unwrap().path());
    }
}

fn print_path_and_children(p: &Path) {
    if p.is_file() {
        println!("{}", p.canonicalize().unwrap().display());
    } else if p.is_dir() {
        print_dir(p)
    } else {
        panic!("unknown path type {}", p.display());
    }
}