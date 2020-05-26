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
}
