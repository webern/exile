use std::path::PathBuf;

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

    // println!("{}", serde_json::to_string_pretty(&t).unwrap());
    //
    // let x = Metadata {
    //     description: "x".to_string(),
    //     syntax: Syntax::Good {},
    //     expected: Some(xdoc::Document::new()),
    // };
    //
    // println!("{}", serde_json::to_string_pretty(&x).unwrap());
}
