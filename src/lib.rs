/*!
![build](https://github.com/webern/exile/workflows/exile%20ci/badge.svg)

`exile` is a Rust library for reading and writing XML.

The goal, at least initially, is to provide an abstract syntax tree of an XML file.
As such, this is a Exile is a dom parser and loads the complete contents of the document into memory.

Currently supported:
- Attributes
- CDATA Sections
- Comment Parsing
- Elements
- Processing Instructions
- Text Nodes
- UTF-8
- Whitespace Normalization

Not Supported:
- Doctypes
- Entities
- Entity References
- Other Encodings
- Whitesace Preservation: All text nodes are treated as if whitespace `collapse` were in-effect.

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
    println!("element name: {}", child.name());
    if let Some(attribute) = child.attribute("name") {
        println!("name attribute: {}", attribute);
    }
}

// we can create an index of elements
let index = doc.index();

// the element at index 2 is <thing>bar</thing>
let thing = index.element(2).unwrap();

// the parent of index 2 is <root>
let root = index.parent(&thing).unwrap();

assert_eq!("bar", thing.text().unwrap());
assert_eq!("root", root.name());
```

Authoring XML looks like this.

```
use exile::{Document, Element, Node};
let mut root = Element::from_name("my_root");
root.add_attribute("foo", "bar");
let mut child = Element::from_name("my_child");
child.add_text("Hello World!");
root.add_child(child);
let doc = Document::from_root(root);
println!("{}", doc.to_string());
```

The above program prints:

```xml
<my_root foo="bar">
  <my_child>Hello World!</my_child>
</my_root>
```

*/

#![deny(rust_2018_idioms)]
#![deny(missing_docs, unused_imports)]

use std::path::Path;

pub use crate::parser::ParseError;
pub use crate::xdoc::{
    Declaration, Document, Element, Encoding, Index, Misc, Node, Pi, Version, WriteOpts,
};
pub use crate::xmlns::{Namespace, NcName, NsErr, QName};

/// The `error` module defines the error types for this library.
#[macro_use]
mod macros;
mod constants;
pub mod error;
pub mod parser;
mod xdoc;
mod xmlns;

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
    assert_eq!("r", root.name());
    assert_eq!(1, root.nodes_len());
    let child = root.first_node().unwrap();
    if let Node::Element(element) = child {
        assert_eq!("a", element.name());
        let attribute_value = element.attribute("b").unwrap();
        assert_eq!("c", attribute_value);
    } else {
        panic!("expected element but found a different type of node")
    }
}
