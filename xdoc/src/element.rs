use std::io::Write;

use crate::error::Result;
use crate::write_ops::write_attribute_value;
use crate::{Node, OrdMap, WriteOpts};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Element {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: OrdMap,
    pub nodes: Vec<Node>,
}

impl Element {
    pub fn from_name<S: AsRef<str>>(name: S) -> Self {
        Element {
            namespace: None,
            name: name.as_ref().into(),
            attributes: Default::default(),
            nodes: vec![],
        }
    }

    /// Returns the 'child' elements of the current element. Consider the XML document:
    /// ```xml
    /// <r>
    ///   <a/>
    ///   <b/>
    /// </r>
    /// ```
    /// r's `children()` function would return an iterator over 'a' and 'b'.
    /// Text nodes, processing instructions and comments are skipped/ignored by the iterator.
    pub fn children(&self) -> impl Iterator<Item = &Element> {
        self.nodes.iter().filter_map(|n| {
            if let Node::Element(element) = n {
                return Some(element);
            }
            None
        })
    }

    /// Add an element as a child of this element.
    pub fn add_child(&mut self, element: Element) {
        self.nodes.push(Node::Element(element))
    }

    pub fn fullname(&self) -> String {
        if let Some(ns) = &self.namespace {
            if !ns.is_empty() {
                return format!("{}:{}", ns, self.name);
            }
        }
        self.name.clone()
    }

    pub fn write<W>(&self, writer: &mut W, opts: &WriteOpts, depth: usize) -> Result<()>
    where
        W: Write,
    {
        if let Err(e) = self.check() {
            return wrap!(e);
        }
        // Somewhat hacky, but if it's the root element then the caller needs to decide whether or
        // not a newline precededs the element. For all other elements a newline should be OK.
        if depth != 0 {
            if let Err(e) = opts.newline(writer) {
                return wrap!(e);
            }
        }
        if let Err(e) = opts.indent(writer, depth) {
            return wrap!(e);
        }
        if let Err(e) = write!(writer, "<") {
            return wrap!(e);
        }
        if let Some(ns) = &self.namespace {
            if !ns.is_empty() {
                if let Err(e) = write!(writer, "{}:", ns) {
                    return wrap!(e);
                }
            }
        }
        if let Err(e) = write!(writer, "{}", self.name) {
            return wrap!(e);
        }

        let attribute_keys = self
            .attributes
            .map()
            .keys()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>();
        for &k in attribute_keys.iter() {
            if let Err(e) = write!(writer, " {}=\"", &k) {
                return wrap!(e);
            }
            if let Some(val) = self.attributes.map().get(k) {
                better_wrap!(write_attribute_value(val, writer, opts))?;
            }
            if let Err(e) = write!(writer, "\"") {
                return wrap!(e);
            }
        }

        if self.nodes.is_empty() {
            if let Err(e) = write!(writer, "/>") {
                return wrap!(e);
            } else {
                // if let Err(e) = opts.newline(writer) {
                //     return wrap!(e);
                // }
                return Ok(());
            }
        } else {
            // if let Err(e) = opts.indent(writer, depth) {
            //     return wrap!(e);
            // }
            if let Err(e) = write!(writer, ">") {
                return wrap!(e);
            }
        }
        // if let Err(e) = opts.newline(writer) {
        //     return wrap!(e);
        // }

        for node in self.nodes.iter() {
            if let Err(e) = node.write(writer, opts, depth + 1) {
                // TODO - this may explode with recursive wrapping
                return wrap!(e);
            }
        }

        // Closing Tag
        if let Err(e) = write!(writer, "</") {
            return wrap!(e);
        }
        if let Some(ns) = &self.namespace {
            if !ns.is_empty() {
                if let Err(e) = write!(writer, "{}:", ns) {
                    return wrap!(e);
                }
            }
        }
        if let Err(e) = write!(writer, "{}", self.name) {
            return wrap!(e);
        }
        if let Err(e) = write!(writer, ">") {
            return wrap!(e);
        }
        if let Err(e) = opts.newline(writer) {
            return wrap!(e);
        }
        Ok(())
    }

    fn check(&self) -> Result<()> {
        if self.name.is_empty() {
            return raise!("Empty element name.");
        }
        if let Some(ns) = &self.namespace {
            if ns.is_empty() {
                return raise!("Namespace should not be empty when the option is 'some'.");
            }
        }
        for attribute_key in self.attributes.map().keys() {
            if attribute_key.is_empty() {
                return raise!("Empty attribute name encountered.");
            }
        }
        Ok(())
    }
}

#[test]
fn test_children() {
    let mut root = Element::from_name("root");
    root.add_child(Element::from_name("a"));
    root.nodes.push(Node::Text("some text".into()));
    root.add_child(Element::from_name("b"));
    let mut children = root.children();
    let first_child = children.next();
    assert_eq!("a", first_child.unwrap().name.as_str());
    let second_child = children.next();
    assert_eq!("b", second_child.unwrap().name.as_str());
    let end_none = children.next();
    assert!(end_none.is_none());
}