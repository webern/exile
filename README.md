# exile

Current version: 0.0.3

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
- Whitespace Normalization

Not Supported:
- Entities
- Entity References
- Doctypes
- Comment Parsing
- Other Encodings
- Whitesace Preservation: All text nodes are treated as if whitespace `collapse` were in-effect.

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
    println!("element name: {}", child.name());
    if let Some(attribute) = child.attribute("name") {
        println!("name attribute: {}", attribute);
    }
}
```

Authoring XML looks like this.

```rust
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
