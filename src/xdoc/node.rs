use std::io::Write;

use crate::xdoc::error::Result;
use crate::xdoc::write_ops::write_element_string;
use crate::{Element, WriteOpts};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
/// Represents a Node in an XML Document. The Document consists of a recursive nesting of these.
pub enum Node {
    /// `<element/>`
    Element(Element),

    /// Text data in an element, i.e. `<x>hello &lt;</x>` where the `Text` is `hello <`.
    Text(String),

    // TODO - support CDATA https://github.com/webern/exile/issues/28
    /// `<![CDATA[text]]>` - not implemented
    CData(String),

    /// Represents comments, processing instructions and whitespace.
    Misc(Misc),

    // TODO - support doctypes https://github.com/webern/exile/issues/22
    /// `<!DOCTYPE doc>` - not implemented
    DocType(String),
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
            Node::Element(data) => data.write(writer, opts, depth),
            Node::Text(s) => {
                write_element_string(s.as_str(), writer, opts, depth)?;
                Ok(())
            }
            Node::CData(_) => {
                Ok(()) /*TODO - implement*/
            }
            Node::Misc(m) => m.write(writer, opts, depth),
            Node::DocType(_) => {
                Ok(()) /*TODO - implement*/
            }
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

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
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
