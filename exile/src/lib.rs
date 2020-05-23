/*!
Half baked work on XML in Rust.
*/
#[macro_use]
extern crate log;

pub use parser::parse_str;
pub use structure::Attribute;
pub use structure::Element;
pub use structure::ElementContent;

// pub use structure::Namespace;

pub mod error;
mod parser;
mod structure;
