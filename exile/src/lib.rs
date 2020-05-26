/*!

`exile` is a Rust library for reading and writing XML.

The goal is to provide a useful abstraction over XML and get better at writing Rust.
The state of the library is 'pre-alpha', see the GitHub issues and milestones for work planned.

## Example

Using the library looks like this

```
    let xml = r#"
    <root>
      <thing name="foo"/>
      <thing>bar</thing>
    </root>
    "#;
    let doc = exile::parse(xml).unwrap();
    for child in doc.root().children() {
        println!("element name: {}", child.name);
        if let Some(attribute) = child.attributes.map().get("name") {
            println!("name attribute: {}", attribute);
        }
    }
```

## Work to Do

 * [ ] [v0.0.0 MVP]
 * [ ] [v0.0.1 Interface]
 * [ ] [v0.0.2 Conformance]

[Issues not assigned to a milestone]

[v0.0.0 MVP]: https://github.com/webern/exile/milestone/1
[v0.0.1 Interface]: https://github.com/webern/exile/milestone/2
[v0.0.2 Conformance]: https://github.com/webern/exile/milestone/2
[Issues not assigned to a milestone]: https://github.com/webern/exile/issues?q=is%3Aissue+is%3Aopen+no%3Amilestone

*/

#![warn(missing_docs)]

pub use xdoc::{Document, Element, Node};

/// The `error` module defines the error types for this library.
#[macro_use]
pub mod error;
mod parser;

/// Currently this is the only way to parse an XML document.
/// TODO - streaming https://github.com/webern/exile/issues/20
pub fn parse(xml: &str) -> crate::error::Result<Document> {
    parser::document_from_string(xml)
}

#[test]
fn simple_document_test() {
    let xml = r#"
    <r>
      <a b="c"/>
    </r>
    "#;
    let doc = parse(xml).unwrap();
    let root = doc.root();
    assert_eq!("r", root.name.as_str());
    assert_eq!(1, root.nodes.len());
    let child = root.nodes.first().unwrap();
    if let Node::Element(element) = child {
        assert_eq!("a", element.name.as_str());
        let attribute_value = element.attributes.map().get("b").unwrap();
        assert_eq!("c", attribute_value.as_str());
    } else {
        panic!("expected element but found a different type of node")
    }
}
