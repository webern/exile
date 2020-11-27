use std::io::Write;

use crate::xdoc::cdata::write_cdata;
use crate::xdoc::error::Result;
use crate::xdoc::write_ops::write_element_text;
use crate::{Element, WriteOpts, PI};

#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Hash)]
/// Represents a Node in an XML Document. The Document consists of a recursive nesting of these.
pub enum Node {
    /// `<![CDATA[text]]>`
    CData(String),

    /// Comment, e.g. `<!--some comment-->`
    Comment(String),

    // TODO - support doctypes https://github.com/webern/exile/issues/22
    /// `<!DOCTYPE doc>` - not implemented
    DocType(String),

    /// `<element/>`
    Element(Element),

    /// Processing Instruction, e.g. `<?target data?>`
    PI(PI),

    /// Text data in an element, i.e. `<x>hello &lt;</x>` where the `Text` is `hello <`.
    Text(String),
}

impl Default for Node {
    fn default() -> Self {
        Node::Element(crate::Element::default())
    }
}

impl Node {
    /// Serialize the XML Document to a `Write` stream.
    pub fn write<W>(&self, writer: &mut W, opts: &WriteOpts, depth: usize) -> Result<()>
    where
        W: Write,
    {
        match self {
            Node::CData(cdata) => write_cdata(cdata, writer),
            Node::Comment(_) => panic!("comments unsupported"),
            Node::DocType(_) => panic!("doctypes unsupported"),
            Node::Element(data) => data.write(writer, opts, depth),
            Node::PI(pi) => pi.write(writer, opts, depth),
            Node::Text(s) => write_element_text(s.as_str(), writer, opts, depth),
        }
    }

    /// Returns true if this node is either a Node::Text or a Node::CData.
    pub fn is_text(&self) -> bool {
        match self {
            Node::Text(_) => true,
            Node::CData(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, Ord, PartialEq, Hash)]
// TODO - support Whitespace https://github.com/webern/exile/issues/55
/// Represents a "Misc" entry, which is a Processing Instruction (PI), Comment, or Whitespace
pub enum Misc {
    // TODO - support comments https://github.com/webern/exile/issues/22
    /// `<!-- comment -->` - not implemented
    Comment(String),
    /// ProcessingInstruction, e.g. `<?target whatever?>` - not implemented
    PI(crate::PI),
}

impl Default for Misc {
    fn default() -> Self {
        Misc::Comment("".to_owned())
    }
}

impl Misc {
    /// Serialize the XML Document to a `Write` stream.
    pub fn write<W>(&self, writer: &mut W, opts: &WriteOpts, depth: usize) -> Result<()>
    where
        W: Write,
    {
        match self {
            // TODO - implement comments https://github.com/webern/exile/issues/27
            Misc::Comment(_) => unimplemented!(),
            Misc::PI(pi) => pi.write(writer, opts, depth),
        }
    }
}
