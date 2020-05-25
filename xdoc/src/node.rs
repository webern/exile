use std::io::Write;

use crate::doc::WriteOpts;
use crate::error::Result;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Node {
    // <element>
    Element(crate::ElementData),

    // normal text data, i.e. 'text &lt;'
    String(String),

    // <![CDATA[text]]>
    CData(String),

    // <!-- comment -->
    Comment(String),

    // <?target data1 data2 data3?>'
    ProcessingInstruction(crate::PIData),

    // <!DOCTYPE doc> Contents are a blob
    DocType(String),
}

impl Default for Node {
    fn default() -> Self {
        Node::Element(crate::ElementData::default())
    }
}

impl Node {
    pub fn write<W>(&self, writer: &mut W, opts: &WriteOpts, depth: usize) -> Result<()>
    where
        W: Write,
    {
        match self {
            Node::Element(data) => data.write(writer, opts, depth),
            Node::String(s) => {
                write_element_string(s.as_str(), writer, opts, depth)?;
                Ok(())
            }
            Node::CData(_) => {
                Ok(()) /*TODO - implement*/
            }
            Node::Comment(_) => {
                Ok(()) /*TODO - implement*/
            }
            Node::ProcessingInstruction(_) => {
                Ok(()) /*TODO - implement*/
            }
            Node::DocType(_) => {
                Ok(()) /*TODO - implement*/
            }
        }
    }
}

fn write_element_string<W, S>(s: S, writer: &mut W, _opts: &WriteOpts, _depth: usize) -> Result<()>
where
    W: Write,
    S: AsRef<str>,
{
    // TODO - support additional escapes https://github.com/webern/exile/issues/44
    for c in s.as_ref().chars() {
        match c {
            '<' => better_wrap!(write!(writer, "&lt;"))?,
            '>' => better_wrap!(write!(writer, "&gt;"))?,
            '&' => better_wrap!(write!(writer, "&amp;"))?,
            _ => better_wrap!(write!(writer, "{}", c))?,
        }
    }
    Ok(())
}
