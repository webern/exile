# exile

Current version: 0.0.0


`exile` is a Rust library for reading and writing XML.

The goal is to provide a useful abstraction over XML and get better at writing Rust.
The state of the library is 'pre-alpha', see the GitHub issues and milestones for work planned.

### Example

Using the library looks like this

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

### Work to Do

 * [ ] [v0.0.0 MVP]
 * [ ] [v0.0.1 Interface]
 * [ ] [v0.0.2 Conformance]

[Issues not assigned to a milestone]

[v0.0.0 MVP]: https://github.com/webern/exile/milestone/1
[v0.0.1 Interface]: https://github.com/webern/exile/milestone/2
[v0.0.2 Conformance]: https://github.com/webern/exile/milestone/2
[Issues not assigned to a milestone]: https://github.com/webern/exile/issues?q=is%3Aissue+is%3Aopen+no%3Amilestone
