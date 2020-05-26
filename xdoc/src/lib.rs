#![warn(missing_docs)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::hash::Hash;

pub use doc::Document;
pub use doc::{Declaration, Encoding, Version};
pub use element::Element;
pub use node::Node;
pub use nodes::Nodes;
pub use ord_map::OrdMap;
pub use write_ops::{Newline, WriteOpts};

pub use crate::error::Result;

#[macro_use]
pub mod error;

mod doc;
mod element;
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

#[cfg(test)]
mod tests {
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
