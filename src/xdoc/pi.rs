use core::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Write};

use crate::xdoc::error::{Result, XDocErr};
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
pub struct Pi {
    /// The processing instruction target.
    target: String,
    /// The processing instruction data.
    data: String,
}

impl Pi {
    /// Create a new processing instruction.
    pub fn new<S1, S2>(target: S1, data: S2) -> Result<Self>
    where
        S1: Into<String> + AsRef<str>,
        S2: Into<String> + AsRef<str>,
    {
        if !pi_str_ok(target.as_ref()) {
            // TODO - improve this error mess
            return Err(XDocErr {
                message: format!(
                    "invalid processing instruction target '{}'",
                    target.as_ref()
                ),
                file: file!().into(),
                line: line!() as u64,
                source: None,
            });
        }
        if !pi_str_ok(data.as_ref()) {
            // TODO - improve this error mess
            return Err(XDocErr {
                message: format!("invalid processing instruction data '{}'", target.as_ref()),
                file: file!().into(),
                line: line!() as u64,
                source: None,
            });
        }
        // TODO - validate strings
        Ok(Self::new_unchecked(target, data))
    }

    /// Return the target from a processing instruction, e.g. in `<?foo bar baz?>`, `foo` is the
    /// target.
    pub fn target(&self) -> &String {
        &self.target
    }

    /// Return the data from a processing instruction, e.g. in `<?foo bar baz?>`, `bar baz` is the
    /// data.
    pub fn data(&self) -> &String {
        &self.data
    }

    /// Write the processing instruction to the `Write` object.
    pub fn write<W>(&self, writer: &mut W, opts: &WriteOpts, depth: usize) -> Result<()>
    where
        W: Write,
    {
        opts.indent(writer, depth)?;
        xwrite!(writer, "<?{}", &self.target)?;
        if !self.data.is_empty() {
            xwrite!(writer, " {}", self.data)?;
        }
        xwrite!(writer, "?>")?;
        Ok(())
    }

    pub(crate) fn new_unchecked<S1, S2>(target: S1, data: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            target: target.into(),
            data: data.into(),
        }
    }
}

fn pi_str_ok<S: AsRef<str>>(s: S) -> bool {
    !s.as_ref().contains("?>")
}

impl Display for Pi {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut c = Cursor::new(Vec::new());
        if self.write(&mut c, &WriteOpts::default(), 0).is_err() {
            return write!(f, "<?error?>");
        }
        let data = c.into_inner();
        let data_str = std::str::from_utf8(data.as_slice()).unwrap_or("<?error?>");
        write!(f, "{}", data_str)
    }
}

#[test]
fn pi_test_simple() {
    let pi = Pi::new("thetarget", "dat1 dat2").unwrap();
    let got = pi.to_string();
    let want = "<?thetarget dat1 dat2?>";
    assert_eq!(got, want);
}

#[test]
fn pi_test_empty() {
    let pi = Pi::new("x", "").unwrap();
    let got = pi.to_string();
    let want = "<?x?>";
    assert_eq!(got, want);
}

#[test]
fn pi_test_bad() {
    let result = Pi::new("x", "da?>t1");
    assert!(result.is_err());
}
