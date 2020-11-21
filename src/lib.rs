/*!
![build](https://github.com/webern/exile/workflows/exile%20ci/badge.svg)

`exile` is a Rust library for reading and writing XML.

The goal, at least initially, is to provide an abstract syntax tree of an XML file.
As such, this is a Exile is a dom parser and loads the complete contents of the document into memory.

Currently supported:
- Elements
- Attributes
- Text Nodes
- Processing Instructions
- UTF-8

Not Supported:
- Entities
- Entity References
- Doctypes
- Comment Parsing
- Other Encodings
- Whitespace Preservation

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
*/

#![deny(rust_2018_idioms)]
#![deny(missing_docs, unused_imports)]

use std::path::Path;

pub use crate::xdoc::{
    Declaration, Document, Element, Encoding, Misc, Name, Node, Version, WriteOpts, PI,
};

/// The `error` module defines the error types for this library.
#[macro_use]
pub mod error;
mod parser;
mod xdoc;

/// Parse an XML file held in string contents.
pub fn parse<S: AsRef<str>>(xml: S) -> crate::error::Result<Document> {
    parser::document_from_string(xml)
}

/// Load a document from a file.
pub fn load<P: AsRef<Path>>(path: P) -> crate::error::Result<Document> {
    parser::document_from_file(path)
}

// TODO - streaming https://github.com/webern/exile/issues/20

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
        let attribute_value = element.attribute("b").unwrap();
        assert_eq!("c", attribute_value);
    } else {
        panic!("expected element but found a different type of node")
    }
}
