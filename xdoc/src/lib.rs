#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::hash::Hash;
use std::io::Write;

pub use doc::Document;
pub use node::Node;
pub use nodes::Nodes;
pub use ord_map::OrdMap;

use crate::doc::WriteOpts;
use crate::error::Result;

#[macro_use]
pub mod error;

mod doc;
mod node;
mod nodes;
mod ord_map;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Name {
    pub namespace: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PIData {}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Attribute {
    key: String,
    value: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElementData {
    pub namespace: Option<String>,
    pub name: String,
    pub attributes: OrdMap,
    pub nodes: Vec<Node>,
}

impl ElementData {
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
                // TODO - escape string
                if let Err(e) = write!(writer, "{}", val) {
                    return wrap!(e);
                }
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
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn structs_test() {
        let mut doc = Document::new();
        doc.root = Node::Element(ElementData {
            namespace: None,
            name: "root-element".to_string(),
            attributes: Default::default(),
            nodes: vec![],
        });
        let mut c = Cursor::new(Vec::new());
        let result = doc.write(&mut c);
        assert!(result.is_ok());
        let data = c.into_inner();
        let data_str = std::str::from_utf8(data.as_slice()).unwrap();
        assert_eq!("<root-element/>", data_str);
    }
}
