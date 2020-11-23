/*!
![build](https://github.com/webern/exile/workflows/exile%20ci/badge.svg)

`xdoc` presents the primitives on an XML DOM.
For example `Element` and `Attribute` are concepts in this library.
It is written in support of the `exile` crate, but kept separate from that crate due to dev-time
compilation dependencies.
Specifically, the `xtest` crate uses `xdoc` to generate tests for `exile`.
The public concepts in `xdoc` are re-exported by `exile`.
*/

#![warn(missing_docs)]

pub use chars::is_whitespace;
pub use doc::Document;
pub use doc::{Declaration, Encoding, Version};
pub use element::Element;
pub(crate) use name::Name;
pub use node::{Misc, Node};
pub use pi::PI;
pub use write_ops::{Newline, WriteOpts};

pub use crate::xdoc::error::Result;

/// The public error type for this library. Macros are for internal use.
#[macro_use]
pub mod error;

mod chars;
mod doc;
mod element;
mod name;
mod node;
pub(crate) mod ord_map;
mod pi;
mod write_ops;

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn structs_test() {
        let doc = Document::from_root(Element::from_name("root-element"));
        let mut c = Cursor::new(Vec::new());
        let result = doc.write(&mut c);
        assert!(result.is_ok());
        let data = c.into_inner();
        let data_str = std::str::from_utf8(data.as_slice()).unwrap();
        assert_eq!("<root-element/>", data_str);
    }
}
