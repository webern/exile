use std::io::Write;

use super::error::Result;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CData(String);

/// Checks if a CData string is valid.
pub(super) fn check_cdata<S: AsRef<str>>(cdata: S) -> Result<()> {
    if cdata.as_ref().contains("]]>") {
        return raise!("CDATA string cannot contain ]]>");
    }
    Ok(())
}

// writes a cdata string
pub(super) fn write_cdata<W, S>(cdata: S, writer: &mut W) -> Result<()>
where
    W: Write,
    S: AsRef<str>,
{
    better_wrap!(write!(writer, "<![CDATA[{}]]>", cdata.as_ref()))
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn check_cdata_1() {
    let input = "some data";
    let result = check_cdata(input);
    assert!(result.is_ok())
}

#[test]
fn check_cdata_2() {
    let input = "some data]]>boo";
    let result = check_cdata(input);
    assert!(result.is_err())
}
