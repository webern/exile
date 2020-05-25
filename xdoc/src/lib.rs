#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::borrow::{Borrow, Cow};
use std::hash::Hash;
use std::io::Write;

pub use doc::Document;
pub use doc::{Declaration, Encoding, Version};
pub use node::Node;
pub use nodes::Nodes;
pub use ord_map::OrdMap;
pub use write_ops::{Newline, WriteOpts};

use crate::error::Result;

#[macro_use]
pub mod error;

mod doc;
mod node;
mod nodes;
mod ord_map;
mod write_ops;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Name {
    pub namespace: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct PIData {
    pub target: String,
    pub instructions: OrdMap,
}

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

    pub fn fullname(&self) -> String {
        if let Some(ns) = &self.namespace {
            if !ns.is_empty() {
                return format!("{}:{}", ns, self.name).into();
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
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::io::Cursor;

    use super::*;

    #[test]
    fn structs_test() {
        let mut doc = Document::new();
        doc.set_root(Element {
            namespace: None,
            name: "root-element".into(),
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

fn write_attribute_value<W, S>(s: S, writer: &mut W, _opts: &WriteOpts) -> Result<()>
where
    W: Write,
    S: AsRef<str>,
{
    // TODO - support single quoted attributes https://github.com/webern/exile/issues/45
    // TODO - support additional escapes https://github.com/webern/exile/issues/44
    for c in s.as_ref().chars() {
        match c {
            '<' => better_wrap!(write!(writer, "&lt;"))?,
            '>' => better_wrap!(write!(writer, "&gt;"))?,
            '&' => better_wrap!(write!(writer, "&amp;"))?,
            '"' => better_wrap!(write!(writer, "&quot;"))?,
            _ => better_wrap!(write!(writer, "{}", c))?,
        }
    }
    Ok(())
}
