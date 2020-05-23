/*!
Half baked work on XML in Rust.

# TODO

 * [x] xdoc: create the ezfile in a test using structs
 * [x] xdoc: write assertions for the ezfile structs
 * [x] xdoc: serialize the ezfile to xml
 * [x] xdoc: assert serialized xml equals a string constant of the xml
 * [ ] xdoc: serialize the ezfile to json
 * [ ] xtest: add the serialized ezfile data to the metadata file as an assertion.
 * [ ] exile: generate an assertion of the ezfile using build.rs
 * [ ] exile: make the parser work so that the ezfile test passes
*/

#[macro_use]
extern crate log;

pub use parser::parse_str;
pub use xdoc::{Document, ElementData, Node};

pub mod error;
mod parser;
