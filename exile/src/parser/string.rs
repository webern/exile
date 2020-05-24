use std::borrow::Cow;

use crate::error::Result;

pub(crate) fn parse_string<'a, S>(s: S) -> Result<Cow<'a, str>>
where
    S: Into<Cow<'a, str>>,
{
    Ok("".into())
}
