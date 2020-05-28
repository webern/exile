use std::io::Write;

use crate::error::Result;
use crate::write_ops::write_element_string;
use crate::{Element, WriteOpts};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "lowercase")
)]
/// Represents a Node in an XML Document. The Document consists of a recursive nesting of these.
pub enum Node {
    /// `<element/>`
    Element(Element),

    /// Text data in an element, i.e. `<x>hello &lt;</x>` where the `Text` is `hello <`.
    Text(String),

    // TODO - support CDATA https://github.com/webern/exile/issues/28
    /// `<![CDATA[text]]>` - not implemented
    CData(String),

    // TODO - support comments https://github.com/webern/exile/issues/22
    /// `<!-- comment -->` - not implemented
    Comment(String),

    // TODO - support pis https://github.com/webern/exile/issues/12
    /// ProcessingInstruction, e.g. `<?target whatever?>` - not implemented
    PI(crate::PIData),

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
            Node::Comment(_) => {
                Ok(()) /*TODO - implement*/
            }
            Node::PI(_) => {
                Ok(()) /*TODO - implement*/
            }
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
