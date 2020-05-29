/*!
`exile` is a Rust library for reading and writing XML.

The goal is to provide a useful abstraction over XML with DOM-like structs.
The state of the library is 'pre-alpha'
See the GitHub issues and [milestones](https://github.com/webern/exile/milestones) for work planned.

# Example

Parsing XML looks like this.

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

Authoring XML looks like this.

```
use exile::{Document, Element, Node};
let mut root = Element::from_name("my_root");
// TODO - improve the interface
root.attributes.mut_map().insert("foo".into(), "bar".into());
let mut child = Element::from_name("my_child");
child.nodes.push(Node::Text("Hello World!".into()));
root.nodes.push(Node::Element(child));
let doc = Document::from_root(root);
println!("{}", doc.to_string());
```

The program above prints:

```xml
<my_root foo="bar">
  <my_child>Hello World!</my_child>
</my_root>
```

# Crates

Development is setup as a cargo workspace with three crates.

  * `xdoc` is the root dependency and contains structs that represent an XML document
  * `xtest` contains test files and helper functions for loading them.
  * `exile` is the public-facing crate and includes the parser.

# Work to Do

Current status is 'pre-mvp'. v0.0.0 will be able to parse basic XML documents into DOM-like
structures and serialize them back.

 * [x] [v0.0.0 MVP]
 * [ ] [v0.0.1 Conformance]
 * [ ] [v0.0.2 Interface]

[Issues not assigned to a milestone]

[v0.0.0 MVP]: https://github.com/webern/exile/milestone/1
[v0.0.1 Conformance]: https://github.com/webern/exile/milestone/3
[v0.0.2 Interface]: https://github.com/webern/exile/milestone/2
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
