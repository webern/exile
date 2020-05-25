use std::io::Write;

use crate::error::Result;

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
pub enum Newline {
    None,
    Newline,
    Windows,
}

impl Default for Newline {
    fn default() -> Self {
        Newline::Newline
    }
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

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct WriteOpts {
    pub indent: Indent,
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
        let s = std::iter::repeat(s).take(num).collect::<String>();
        if let Err(e) = write!(writer, "{}", s) {
            return wrap!(e);
        }
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
                if let Err(e) = Self::write_repeatedly(writer, depth * n, " ") {
                    return wrap!(e);
                }
            }
            Indent::Tab => {
                if let Err(e) = Self::write_repeatedly(writer, depth, "\t") {
                    return wrap!(e);
                }
            }
        }
        Ok(())
    }

    pub(crate) fn newline<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        if let Err(e) = write!(writer, "{}", self.newline_str()) {
            return wrap!(e);
        }
        Ok(())
    }
}
