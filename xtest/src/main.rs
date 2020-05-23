use xdoc::Name;
use xtest::{Assertion, Expected, Syntax, TestMetadata};

fn main() {
    let t = TestMetadata {
        description: "the things are described here".to_string(),
        syntax: Syntax::Bad {
            character_position: 21,
            line: 2,
            column: 10,
        },
        assertions: None,
    };

    println!("{}", serde_json::to_string_pretty(&t).unwrap());

    let x = TestMetadata {
        description: "x".to_string(),
        syntax: Syntax::Good {},
        assertions: Some(vec![Assertion {
            path: "/cats/cat/name".to_string(),
            expected: Expected::Attribute {
                value: "bones".to_string(),
            },
        }]),
    };

    println!("{}", serde_json::to_string_pretty(&x).unwrap());

    let name = Name {
        namespace: Some("hoop".to_string()),
        name: "bloop".to_string(),
    };

    println!("{}", serde_json::to_string_pretty(&name).unwrap());
}
