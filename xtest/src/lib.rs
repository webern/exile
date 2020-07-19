/*!
 ![build](https://github.com/webern/exile/workflows/exile%20ci/badge.svg)

 `xtest` provides a set of XML files for testing. Each file comes with a manifest (in JSON
 format) which describes the XML file. For example, some XML files include intentional syntax
 errors, and the the accompanying JSON manifest will make this apparent.

*/

#[macro_use]
extern crate serde;

pub use metadata::Metadata;
pub use xml_file::{Syntax, XmlFile};
pub use {io::load, io::load_all};

pub mod gen;
mod io;
mod metadata;
mod xml_file;
