use core::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Write};

use crate::xdoc::error::Result;
use crate::xdoc::WriteOpts;

/// Represents a Processing Instruction (PI) in an XML document.
///
/// ## XML Spec
///
/// https://www.w3.org/TR/2006/REC-xml11-20060816/#sec-pi
/// [Definition: Processing instructions (PIs) allow documents to contain instructions for
/// applications.]
///
/// Processing Instructions
///
/// `[16] PI       ::= '<?' PITarget (S (Char* - (Char* '?>' Char*)))? '?>'`
/// `[17] PITarget ::= Name - (('X' | 'x') ('M' | 'm') ('L' | 'l'))`
///
/// > PIs are not part of the document's character data, but must be passed through to the
/// > application. The PI begins with a target (PITarget) used to identify the application to which
/// > the instruction is directed. The target names "XML", "xml", and so on are reserved for
/// > standardization in this or future versions of this specification. The XML Notation mechanism
/// > may be used for formal declaration of PI targets. Parameter entity references must not be
/// > recognized within processing instructions.
///
#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Ord, Hash, Default)]
pub struct PI {
    /// The processing instruction target.
    pub target: String,
    /// The processing instruction data.
    pub data: String,
}

impl PI {
    /// Write the processing instruction to the `Write` object.
    pub fn write<W>(&self, writer: &mut W, opts: &WriteOpts, depth: usize) -> Result<()>
    where
        W: Write,
    {
        self.check()?;
        opts.indent(writer, depth)?;
        xwrite!(writer, "<?{}", &self.target)?;
        if !self.data.is_empty() {
            xwrite!(writer, " {}", self.data)?;
        }
        xwrite!(writer, "?>")?;
        Ok(())
    }

    fn check(&self) -> Result<()> {
        // TODO - check that the name is compliant
        if self.data.contains("?>") {
            return raise!("Processing instruction data contains '?>'.");
        }
        Ok(())
    }
}

impl Display for PI {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut c = Cursor::new(Vec::new());
        if self.write(&mut c, &WriteOpts::default(), 0).is_err() {
            return write!(f, "<?error?>");
        }
        let data = c.into_inner();
        let data_str = match std::str::from_utf8(data.as_slice()) {
            Ok(s) => s,
            Err(_) => "<?error?>",
        };
        write!(f, "{}", data_str)
    }
}

#[test]
fn pi_test_simple() {
    let mut pi = PI::default();
    pi.target = "thetarget".into();
    pi.data = "dat1 dat2".into();
    let got = pi.to_string();
    let want = "<?thetarget dat1 dat2?>";
    assert_eq!(got, want);
}

#[test]
fn pi_test_empty() {
    let mut pi = PI::default();
    pi.target = "x".into();
    let got = pi.to_string();
    let want = "<?x?>";
    assert_eq!(got, want);
}

#[test]
fn pi_test_bad() {
    let mut pi = PI::default();
    pi.target = "x".into();
    pi.data = "da?>t1".into();
    let got = pi.to_string();
    let want = "<?error?>";
    assert_eq!(got, want);
}
