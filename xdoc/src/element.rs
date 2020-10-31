use std::io::Write;

use crate::error::Result;
use crate::write_ops::write_attribute_value;
use crate::{Node, OrdMap, WriteOpts};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
/// Represents an Element in an XML Document.
pub struct Element {
    /// The namespace of this element. e.g. in `foo:bar`, `foo` is the namespace.
    pub namespace: Option<String>,
    /// The name of this element. e.g. in `foo:bar`, `bar` is the name.
    pub name: String,
    /// Attributes of this element.
    pub attributes: OrdMap,
    /// Children of this element.
    pub nodes: Vec<Node>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            namespace: None,
            name: "element".to_string(),
            attributes: Default::default(),
            nodes: vec![],
        }
    }
}

impl Element {
    /// Create a new element using the given name.
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

    /// Find the first occurrance specific child element (does not recurse to lower levels of children).
    pub fn child<S: AsRef<str>>(&self, name: S) -> Option<&Element> {
        let name = name.as_ref();
        for child in self.children() {
            if child.name.as_str() == name {
                return Some(child);
            }
        }
        None
    }

    /// Add an element as a child of this element.
    pub fn add_child(&mut self, element: Element) {
        self.nodes.push(Node::Element(element))
    }

    /// The fullname of the element (including both the namespace and the name).
    pub fn fullname(&self) -> String {
        if let Some(ns) = &self.namespace {
            if !ns.is_empty() {
                return format!("{}:{}", ns, self.name);
            }
        }
        self.name.clone()
    }

    // TODO - fullname? figure out namespace stuff
    /// Sets the name of this element.
    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        self.name = name.as_ref().into()
    }

    /// Does this element have any sub elements. For example, if the element is empty or contains
    /// only text and/or pis and/or comments, then false. if the element has elements, then true.
    pub fn has_children(&self) -> bool {
        if self.nodes.is_empty() || self.is_text() {
            return false;
        }
        for node in &self.nodes {
            if let Node::Element(_) = node {
                return true;
            }
        }
        false
    }

    /// Returns true if there is exactly one sub node, and that sub node is either text or cdata.
    pub fn is_text(&self) -> bool {
        if self.nodes.len() == 1 {
            if let Some(node) = self.nodes.first() {
                match node {
                    Node::Text(_) => return true,
                    Node::CData(_) => return true,
                    _ => return false,
                }
            }
        }
        false
    }

    /// Returns the contents of the first `Text` or `CData` node encountered in the element. Useful
    /// for simple 'text' elements like `<something>text is here</something>` where this function
    /// will return `Some("text is here")`.
    pub fn text(&self) -> Option<String> {
        for node in &self.nodes {
            match node {
                Node::Text(s) => return Some(s.clone()),
                Node::CData(s) => return Some(s.clone()),
                Node::Element(_) => return None,
                _ => continue,
            };
        }
        None
    }

    /// Write the element to the `Write` object.
    pub fn write<W>(&self, writer: &mut W, opts: &WriteOpts, depth: usize) -> Result<()>
    where
        W: Write,
    {
        if let Err(e) = self.check() {
            return wrap!(e);
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
                return Ok(());
            }
        } else if let Err(e) = write!(writer, ">") {
            return wrap!(e);
        }

        for (index, node) in self.nodes.iter().enumerate() {
            if index == 0 && !node.is_text() {
                opts.newline(writer)?;
            }
            if let Err(e) = node.write(writer, opts, depth + 1) {
                // TODO - this may explode with recursive wrapping
                return wrap!(e);
            }
            if !node.is_text() {
                opts.newline(writer)?;
            }
        }

        // Closing Tag
        if !self.is_text() {
            opts.indent(writer, depth)?;
        }
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
        // if let Err(e) = opts.newline(writer) {
        //     return wrap!(e);
        // }
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
