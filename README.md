# exile

Current version: 0.0.2

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

## Example

Parsing XML looks like this.

```rust
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

```rust
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