use std::io::Write;

use crate::xdoc::cdata::check_cdata;
use crate::xdoc::error::{Result, XDocErr};
use crate::xdoc::ord_map::OrdMap;
use crate::xdoc::write_ops::write_attribute_value;
use crate::xdoc::Name;
use crate::{Misc, Node, WriteOpts, PI};

#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Hash)]
/// Represents an Element in an XML Document.
pub struct Element {
    /// The name of this element, which may contain a prefix part, such as `ns` in `ns:foo`.
    name: Name,
    /// Attributes of this element.
    attributes: OrdMap,
    /// Children of this element.
    nodes: Vec<Node>,
}

impl Default for Element {
    fn default() -> Self {
        Self::from_name("element")
    }
}

impl Element {
    /// Create a new element using the given name.
    pub fn from_name<S: Into<String>>(name: S) -> Self {
        Element {
            name: Name::new(name.into()),
            attributes: Default::default(),
            nodes: Default::default(),
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

    /// Add a `CDATA` node. Will error if the string contains `]]>` as this cannot be represented in
    /// a `CDATA` node.
    pub fn add_cdata<S: Into<String>>(&mut self, cdata: S) -> Result<()> {
        let cdata = cdata.into();
        check_cdata(cdata.as_str())?;
        self.nodes.push(Node::CData(cdata));
        Ok(())
    }

    /// Get the number of nodes (of any kind) that are children of this node.
    pub fn nodes_len(&self) -> usize {
        self.nodes.len()
    }

    /// Get the first child node (of any kind)
    pub fn first_node(&self) -> Option<&Node> {
        self.nodes.first()
    }

    /// Get the child node (of any kind) at `index`.
    pub fn node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }

    /// The fullname of the element (including both the namespace alias prefix and the name). For
    /// example, if the name of this element is `ns:foo`, this function returns `"ns:foo"`.
    /// [`Element::name`] and [`Element:prefix`] give the parsed sections of the fullname.
    pub fn fullname(&self) -> &str {
        self.name.full()
    }

    /// The name of the element without its prefix. For example, if the name of this element is
    /// `ns:foo`, `name()` will return `foo`.
    pub fn name(&self) -> &str {
        self.name.name()
    }

    /// The name of the element's namespace alias prefix. For example, if the name of this element
    /// is `ns:foo`, `prefix()` will return `Some("ns")`.
    pub fn prefix(&self) -> Option<&str> {
        self.name.prefix()
    }

    /// Sets the name of this element without changing the namespace alias prefix. For example, if
    /// the name of this element is `ns:foo` then `set_name("bar")` will change the fullname to
    /// `ns:bar`.
    pub fn set_name<S: AsRef<str>>(&mut self, name: S) {
        // TODO check validity of the name, return an error?
        self.name.set_name(name)
    }

    /// Sets the namespace alias prefix of this element without changing the name. For example, if
    /// the name of this element is `ns:foo` then `set_prefix("xyz:) will change the fullname to
    /// `xyz:foo`.
    pub fn set_prefix<S: AsRef<str>>(&mut self, prefix: S) -> Result<()> {
        // TODO check validity of the prefix
        self.name.set_prefix(prefix);
        Ok(())
    }

    /// Sets the fullname of the element. For example, if the name of this element is `ns:foo`, then
    /// `set_fullname("xyz:baz")` will set the fullname to `xyz:baz`. `set_fullname("baz")` will
    /// eliminate any existing namespace alias prefix and set the fullname to `baz`.
    pub fn set_fullname<S: Into<String>>(&mut self, fullname: S) -> Result<()> {
        // TODO check validity of the fullname
        self.name.set_full(fullname);
        Ok(())
    }

    /// Inserts a key-value pair into the attributes map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    ///
    pub fn add_attribute<K, V>(&mut self, key: K, value: V) -> Option<String>
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.attributes
            .mut_map()
            .insert(key.as_ref().into(), value.as_ref().into())
    }

    /// Gets the attribute value at `key`. `None` if an attribute by that name does not exist.
    pub fn attribute<S: AsRef<str>>(&self, key: S) -> Option<&str> {
        self.attributes.map().get(key.as_ref()).map(|s| s.as_str())
    }

    /// Gets the count of attributes.
    pub fn attributes_len(&self) -> usize {
        self.attributes.map().len()
    }

    /// Creates a new element as the last child of this element and returns a mut ref to it.
    pub fn add_new_child(&mut self) -> Result<&mut Element> {
        self.nodes.push(Node::Element(Element::default()));
        let new_node = self.nodes.last_mut().ok_or_else(|| XDocErr {
            message: "the sky is falling".to_string(),
            file: "".to_string(),
            line: 0,
            source: None,
        })?;
        if let Node::Element(new_element) = new_node {
            Ok(new_element)
        } else {
            Err(XDocErr {
                message: "the sky is still falling".to_string(),
                file: "".to_string(),
                line: 0,
                source: None,
            })
        }
    }

    /// Append a text node to this element's nodes.
    pub fn add_text<S: AsRef<str>>(&mut self, text: S) {
        self.nodes.push(Node::Text(text.as_ref().into()))
    }

    /// Append a processing instruction to this element's nodes.
    pub fn add_pi(&mut self, pi: PI) {
        self.nodes.push(Node::Misc(Misc::PI(pi)))
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
            return raise!(e);
        }
        opts.indent(writer, depth)?;
        xwrite!(writer, "<")?;
        xwrite!(writer, "{}", self.fullname())?;

        let attribute_keys = self
            .attributes
            .map()
            .keys()
            .map(|k| k.as_str())
            .collect::<Vec<&str>>();
        for &k in attribute_keys.iter() {
            xwrite!(writer, " {}=\"", &k)?;
            if let Some(val) = self.attributes.map().get(k) {
                write_attribute_value(val, writer, opts)?;
            }
            xwrite!(writer, "\"")?;
        }

        if self.nodes.is_empty() {
            xwrite!(writer, "/>")?;
            return Ok(());
        } else {
            xwrite!(writer, ">")?;
        }

        for (index, node) in self.nodes.iter().enumerate() {
            if index == 0 && !node.is_text() {
                opts.newline(writer)?;
            }
            node.write(writer, opts, depth + 1)?;
            if !node.is_text() {
                opts.newline(writer)?;
            }
        }

        // Closing Tag
        if !self.is_text() {
            opts.indent(writer, depth)?;
        }
        xwrite!(writer, "</{}>", self.fullname())?;
        Ok(())
    }

    fn check(&self) -> std::result::Result<(), &'static str> {
        if self.name.is_empty() {
            return Err("Empty element name.");
        }
        if let Some(ns) = self.prefix() {
            if ns.is_empty() {
                return Err("Namespace should not be empty when the option is 'some'.");
            }
        }
        for attribute_key in self.attributes.map().keys() {
            if attribute_key.is_empty() {
                return Err("Empty attribute name encountered.");
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
