use std::io::Write;

use crate::xdoc::error::Result;

/// The type of newline character to use when writing the XML Document
#[derive(Debug, Default, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub enum Newline {
    /// No newline character.
    None,
    /// The unix/linux newline character `\n`.
    #[default]
    #[allow(clippy::enum_variant_names)]
    Newline,
    /// The Windows newline sequence `\n\r`.
    Windows,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub enum Indent {
    None,
    Spaces(usize),
    Tab,
}

impl Default for Indent {
    fn default() -> Self {
        Indent::Spaces(2)
    }
}

/// Options for controlling how the XML Document is written when serialized.
#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct WriteOpts {
    /// The type of indenting to use when writing the document, i.e. tabs, 2 spaces, 4 spaces.
    pub indent: Indent,
    /// The type of newline to use when writing the document.
    pub newline: Newline,
}

impl WriteOpts {
    fn newline_str(&self) -> &'static str {
        match self.newline {
            Newline::None => "",
            Newline::Newline => "\n",
            Newline::Windows => "\n\r",
        }
    }

    fn write_repeatedly<W>(writer: &mut W, num: usize, s: &str) -> Result<()>
    where
        W: Write,
    {
        xwrite!(writer, "{}", s.repeat(num))?;
        Ok(())
    }

    pub(crate) fn indent<W>(&self, writer: &mut W, depth: usize) -> Result<()>
    where
        W: Write,
    {
        match self.indent {
            Indent::None => {
                return Ok(());
            }
            Indent::Spaces(n) => {
                Self::write_repeatedly(writer, depth * n, " ")?;
            }
            Indent::Tab => {
                Self::write_repeatedly(writer, depth, "\t")?;
            }
        }
        Ok(())
    }

    pub(crate) fn newline<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        xwrite!(writer, "{}", self.newline_str())?;
        Ok(())
    }
}

pub(crate) fn write_attribute_value<W, S>(s: S, writer: &mut W, _opts: &WriteOpts) -> Result<()>
where
    W: Write,
    S: AsRef<str>,
{
    // TODO - support single quoted attributes https://github.com/webern/exile/issues/45
    // TODO - support additional escapes https://github.com/webern/exile/issues/44
    for c in s.as_ref().chars() {
        match c {
            '<' => better_wrap!(write!(writer, "&lt;"))?,
            '>' => better_wrap!(write!(writer, "&gt;"))?,
            '&' => better_wrap!(write!(writer, "&amp;"))?,
            '"' => better_wrap!(write!(writer, "&quot;"))?,
            _ => better_wrap!(write!(writer, "{}", c))?,
        }
    }
    Ok(())
}

// writes a string escaping as necessary for inclusion in an element.
pub(crate) fn write_element_text<W, S>(
    s: S,
    writer: &mut W,
    _opts: &WriteOpts,
    _depth: usize,
) -> Result<()>
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
            '\u{a0}' => better_wrap!(write!(writer, "&#xA0;"))?,
            _ => better_wrap!(write!(writer, "{}", c))?,
        }
    }
    Ok(())
}
