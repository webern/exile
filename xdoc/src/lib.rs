/*!
![build](https://github.com/webern/exile/workflows/exile%20ci/badge.svg)

`xdoc` presents the primitives on an XML Document.
For example `Element` and `Attribute` are structs in this library.
It is written in support of the `exile` crate, but kept separate from that crate due to dev-time
compilation dependencies.
*/

#![warn(missing_docs)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::hash::Hash;

pub use doc::Document;
pub use doc::{Declaration, Encoding, Version};
pub use element::Element;
pub use node::Node;
pub use ord_map::OrdMap;
pub use write_ops::{Newline, WriteOpts};

pub use crate::error::Result;

#[macro_use]
/// The public error type for this library.
pub mod error;

mod doc;
mod element;
mod node;
mod ord_map;
mod write_ops;

/// Represents a Processing Instruction (PI) in an XML document.
#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct PI {
    /// The processing instruction target.
    pub target: String,
    /// The processing instructions.
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
