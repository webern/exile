use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// The error type for errors related to the parsing of namespace strings.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[allow(missing_docs)]
pub enum NsErr {
    Empty,
    InvalidStart(char),
    InvalidChar(char),
    Malformed(&'static str),
}

impl Display for NsErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NsErr::Empty => std::fmt::Display::fmt("empty string not allowed", f),
            NsErr::InvalidStart(c) => write!(f, "invalid start char '{}'", c),
            NsErr::InvalidChar(c) => write!(f, "invalid char '{}'", c),
            NsErr::Malformed(msg) => write!(f, "malformed: {}", msg),
        }
    }
}

impl Error for NsErr {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// NcName

/// https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCName
/// `[4]` NCName ::= (Letter | '_') (NCNameChar)* /* An XML Name, minus the ":" */
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct NcName(String);

impl NcName {
    /// Create a new `NcName` is `s` is valid.
    pub fn new<S: Into<String>>(s: S) -> std::result::Result<Self, NsErr> {
        let s = s.into();
        Ok(s.try_into()?)
    }

    /// Reference the inner string.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Reference the inner string.
    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for NcName {
    type Error = NsErr;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_ncname(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for NcName {
    type Error = NsErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: String = value.into();
        value.try_into()
    }
}

impl Deref for NcName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl AsRef<str> for NcName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for NcName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.0.as_str(), f)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// QName

/// # Qualified Name
///
/// https://www.w3.org/TR/1999/REC-xml-names-19990114/#ns-qualnames
///
/// ```text
/// [6] QName     ::= (Prefix ':')? LocalPart
/// [7] Prefix    ::= NCName
/// [8] LocalPart ::= NCName
/// ```
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct QName<'a> {
    colon: Option<usize>,
    data: Cow<'a, str>,
}

impl QName<'_> {
    /// Returns a new `QName` if the input is valid as a `QName`.
    pub fn new<S: Into<String>>(s: S) -> std::result::Result<Self, NsErr> {
        let s: String = s.into();
        let colon = Self::parse(s.as_str())?;
        Ok(Self {
            colon,
            data: Cow::Owned(s),
        })
    }

    /// Return the prefix part of the qualified name. e.g. if the qualified name is `xyz:root`, the
    /// prefix part is `xyz`. If the qualified name is `foo`, the prefix part is `None`.
    pub fn prefix(&self) -> Option<&str> {
        self.colon.map(|colon| &self.data[..colon])
    }

    /// Return the local part of the qualified name. e.g. if the qualified name is `xyz:root`, the
    /// local part is `root`. If the qualified name is `foo`, the local part is `foo`.
    pub fn local(&self) -> &str {
        if let Some(colon) = self.colon {
            &self.data[colon + 1..]
        } else {
            &self.data
        }
    }

    /// Return the colon-delimited prefix part and local part. e.g. if the prefix part is `xyz` and
    /// the local part is `root`, we will return `xyz:root`. If the prefix part is `None` and the
    /// local part is `None`, we will return `foo`.
    pub fn fullname(&self) -> &str {
        &self.data
    }

    /// Creates a new QName if the input is valid.
    fn parse<S: AsRef<str>>(value: S) -> std::result::Result<Option<usize>, NsErr> {
        let s = value.as_ref();
        let mut colon_index = None;
        for (ix, c) in s.chars().enumerate() {
            if c == ':' {
                if colon_index.is_some() {
                    return Err(NsErr::Malformed("more than one colon encountered in QName"));
                } else {
                    colon_index = Some(ix)
                }
            }
        }

        if let Some(colon) = colon_index {
            let prefix = &s[..colon - 1];
            let local = &s[colon + 1..];
            validate_ncname(prefix)?;
            validate_ncname(local)?;
        } else {
            validate_ncname(s)?;
        }

        Ok(colon_index)
    }
}

impl<'a> QName<'a> {
    /// Returns a new `QName` if the input is valid as a `QName`.
    pub fn new_borrowed(s: &'a str) -> std::result::Result<Self, NsErr> {
        let colon = Self::parse(s)?;
        Ok(Self {
            colon,
            data: Cow::Borrowed(s),
        })
    }
}

impl TryFrom<String> for QName<'_> {
    type Error = NsErr;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        QName::new(value)
    }
}

impl TryFrom<&str> for QName<'_> {
    type Error = NsErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: String = value.into();
        value.try_into()
    }
}

impl Deref for QName<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.fullname()
    }
}

impl AsRef<str> for QName<'_> {
    fn as_ref(&self) -> &str {
        self.fullname()
    }
}

impl Display for QName<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.fullname(), f)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Namespace

/// An XML namespace is defined by a `URI` and has a local name to serve as an alias. If the
/// namespace is declared without a local name, then it serves as a default namespace.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Namespace {
    /// Having read [this](https://danielmiessler.com/study/difference-between-uri-url/), I feel OK
    /// about leaving the `Url` crate out of this and accepting any string as a URI.
    uri: String,
    local: Option<NcName>,
}

impl Default for Namespace {
    fn default() -> Self {
        Self {
            uri: "http://example.com".into(),
            local: Some("example".try_into().unwrap()),
        }
    }
}

impl Namespace {
    /// Create a new `Namespace` object. `uri` is not constrained, but `local` must be a valid
    /// [`NcName`].
    pub fn new<S1, S2>(uri: S1, local: S2) -> std::result::Result<Self, NsErr>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let local = NcName::new(local)?;
        Ok(Self {
            uri: uri.into(),
            local: Some(local),
        })
    }

    /// Create a new `Namespace` object that serves as a default namespace. `uri` is not
    /// constrained.
    pub fn new_default<S>(uri: S) -> std::result::Result<Self, NsErr>
    where
        S: Into<String>,
    {
        Ok(Self {
            uri: uri.into(),
            local: None,
        })
    }

    /// Get the URI.
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Get the local alias.
    pub fn local(&self) -> Option<&str> {
        self.local.as_ref().map(|ncname| ncname.as_str())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Character Sets

fn validate_ncname<S: AsRef<str>>(value: S) -> std::result::Result<(), NsErr> {
    let value = value.as_ref();
    if value.is_empty() {
        return Err(NsErr::Empty);
    }
    let mut first = false;
    for c in value.chars() {
        if first {
            if !is_letter(c) && c != '_' {
                return Err(NsErr::InvalidStart(c));
            }
        } else if !is_ncname_char(c) {
            return Err(NsErr::InvalidChar(c));
        }
        first = false;
    }
    Ok(())
}

// TODO this is wrong? see e.g. https://www.w3.org/TR/xml11/#NT-NameStartChar
/// https://www.w3.org/TR/1999/REC-xml-names-19990114/#NT-NCNameChar
/// `[5] NCNameChar ::= Letter | Digit | '.' | '-' | '_' | CombiningChar | Extender`
fn is_ncname_char(c: char) -> bool {
    is_letter(c)
        || is_digit(c)
        || matches!(c, '.' | '-' | '_')
        || is_combining_char(c)
        || is_extender(c)
}

/// https://www.w3.org/TR/REC-xml/#NT-BaseChar
fn is_base_char(c: char) -> bool {
    matches!(c, '\u{0041}'..='\u{005A}' |
        '\u{0061}'..='\u{007A}' |
        '\u{00C0}'..='\u{00D6}' |
        '\u{00D8}'..='\u{00F6}' |
        '\u{00F8}'..='\u{00FF}' |
        '\u{0100}'..='\u{0131}' |
        '\u{0134}'..='\u{013E}' |
        '\u{0141}'..='\u{0148}' |
        '\u{014A}'..='\u{017E}' |
        '\u{0180}'..='\u{01C3}' |
        '\u{01CD}'..='\u{01F0}' |
        '\u{01F4}'..='\u{01F5}' |
        '\u{01FA}'..='\u{0217}' |
        '\u{0250}'..='\u{02A8}' |
        '\u{02BB}'..='\u{02C1}' |
        '\u{0386}' |
        '\u{0388}'..='\u{038A}' |
        '\u{038C}' |
        '\u{038E}'..='\u{03A1}' |
        '\u{03A3}'..='\u{03CE}' |
        '\u{03D0}'..='\u{03D6}' |
        '\u{03DA}' |
        '\u{03DC}' |
        '\u{03DE}' |
        '\u{03E0}' |
        '\u{03E2}'..='\u{03F3}' |
        '\u{0401}'..='\u{040C}' |
        '\u{040E}'..='\u{044F}' |
        '\u{0451}'..='\u{045C}' |
        '\u{045E}'..='\u{0481}' |
        '\u{0490}'..='\u{04C4}' |
        '\u{04C7}'..='\u{04C8}' |
        '\u{04CB}'..='\u{04CC}' |
        '\u{04D0}'..='\u{04EB}' |
        '\u{04EE}'..='\u{04F5}' |
        '\u{04F8}'..='\u{04F9}' |
        '\u{0531}'..='\u{0556}' |
        '\u{0559}' |
        '\u{0561}'..='\u{0586}' |
        '\u{05D0}'..='\u{05EA}' |
        '\u{05F0}'..='\u{05F2}' |
        '\u{0621}'..='\u{063A}' |
        '\u{0641}'..='\u{064A}' |
        '\u{0671}'..='\u{06B7}' |
        '\u{06BA}'..='\u{06BE}' |
        '\u{06C0}'..='\u{06CE}' |
        '\u{06D0}'..='\u{06D3}' |
        '\u{06D5}' |
        '\u{06E5}'..='\u{06E6}' |
        '\u{0905}'..='\u{0939}' |
        '\u{093D}' |
        '\u{0958}'..='\u{0961}' |
        '\u{0985}'..='\u{098C}' |
        '\u{098F}'..='\u{0990}' |
        '\u{0993}'..='\u{09A8}' |
        '\u{09AA}'..='\u{09B0}' |
        '\u{09B2}' |
        '\u{09B6}'..='\u{09B9}' |
        '\u{09DC}'..='\u{09DD}' |
        '\u{09DF}'..='\u{09E1}' |
        '\u{09F0}'..='\u{09F1}' |
        '\u{0A05}'..='\u{0A0A}' |
        '\u{0A0F}'..='\u{0A10}' |
        '\u{0A13}'..='\u{0A28}' |
        '\u{0A2A}'..='\u{0A30}' |
        '\u{0A32}'..='\u{0A33}' |
        '\u{0A35}'..='\u{0A36}' |
        '\u{0A38}'..='\u{0A39}' |
        '\u{0A59}'..='\u{0A5C}' |
        '\u{0A5E}' |
        '\u{0A72}'..='\u{0A74}' |
        '\u{0A85}'..='\u{0A8B}' |
        '\u{0A8D}' |
        '\u{0A8F}'..='\u{0A91}' |
        '\u{0A93}'..='\u{0AA8}' |
        '\u{0AAA}'..='\u{0AB0}' |
        '\u{0AB2}'..='\u{0AB3}' |
        '\u{0AB5}'..='\u{0AB9}' |
        '\u{0ABD}' |
        '\u{0AE0}' |
        '\u{0B05}'..='\u{0B0C}' |
        '\u{0B0F}'..='\u{0B10}' |
        '\u{0B13}'..='\u{0B28}' |
        '\u{0B2A}'..='\u{0B30}' |
        '\u{0B32}'..='\u{0B33}' |
        '\u{0B36}'..='\u{0B39}' |
        '\u{0B3D}' |
        '\u{0B5C}'..='\u{0B5D}' |
        '\u{0B5F}'..='\u{0B61}' |
        '\u{0B85}'..='\u{0B8A}' |
        '\u{0B8E}'..='\u{0B90}' |
        '\u{0B92}'..='\u{0B95}' |
        '\u{0B99}'..='\u{0B9A}' |
        '\u{0B9C}' |
        '\u{0B9E}'..='\u{0B9F}' |
        '\u{0BA3}'..='\u{0BA4}' |
        '\u{0BA8}'..='\u{0BAA}' |
        '\u{0BAE}'..='\u{0BB5}' |
        '\u{0BB7}'..='\u{0BB9}' |
        '\u{0C05}'..='\u{0C0C}' |
        '\u{0C0E}'..='\u{0C10}' |
        '\u{0C12}'..='\u{0C28}' |
        '\u{0C2A}'..='\u{0C33}' |
        '\u{0C35}'..='\u{0C39}' |
        '\u{0C60}'..='\u{0C61}' |
        '\u{0C85}'..='\u{0C8C}' |
        '\u{0C8E}'..='\u{0C90}' |
        '\u{0C92}'..='\u{0CA8}' |
        '\u{0CAA}'..='\u{0CB3}' |
        '\u{0CB5}'..='\u{0CB9}' |
        '\u{0CDE}' |
        '\u{0CE0}'..='\u{0CE1}' |
        '\u{0D05}'..='\u{0D0C}' |
        '\u{0D0E}'..='\u{0D10}' |
        '\u{0D12}'..='\u{0D28}' |
        '\u{0D2A}'..='\u{0D39}' |
        '\u{0D60}'..='\u{0D61}' |
        '\u{0E01}'..='\u{0E2E}' |
        '\u{0E30}' |
        '\u{0E32}'..='\u{0E33}' |
        '\u{0E40}'..='\u{0E45}' |
        '\u{0E81}'..='\u{0E82}' |
        '\u{0E84}' |
        '\u{0E87}'..='\u{0E88}' |
        '\u{0E8A}' |
        '\u{0E8D}' |
        '\u{0E94}'..='\u{0E97}' |
        '\u{0E99}'..='\u{0E9F}' |
        '\u{0EA1}'..='\u{0EA3}' |
        '\u{0EA5}' |
        '\u{0EA7}' |
        '\u{0EAA}'..='\u{0EAB}' |
        '\u{0EAD}'..='\u{0EAE}' |
        '\u{0EB0}' |
        '\u{0EB2}'..='\u{0EB3}' |
        '\u{0EBD}' |
        '\u{0EC0}'..='\u{0EC4}' |
        '\u{0F40}'..='\u{0F47}' |
        '\u{0F49}'..='\u{0F69}' |
        '\u{10A0}'..='\u{10C5}' |
        '\u{10D0}'..='\u{10F6}' |
        '\u{1100}' |
        '\u{1102}'..='\u{1103}' |
        '\u{1105}'..='\u{1107}' |
        '\u{1109}' |
        '\u{110B}'..='\u{110C}' |
        '\u{110E}'..='\u{1112}' |
        '\u{113C}' |
        '\u{113E}' |
        '\u{1140}' |
        '\u{114C}' |
        '\u{114E}' |
        '\u{1150}' |
        '\u{1154}'..='\u{1155}' |
        '\u{1159}' |
        '\u{115F}'..='\u{1161}' |
        '\u{1163}' |
        '\u{1165}' |
        '\u{1167}' |
        '\u{1169}' |
        '\u{116D}'..='\u{116E}' |
        '\u{1172}'..='\u{1173}' |
        '\u{1175}' |
        '\u{119E}' |
        '\u{11A8}' |
        '\u{11AB}' |
        '\u{11AE}'..='\u{11AF}' |
        '\u{11B7}'..='\u{11B8}' |
        '\u{11BA}' |
        '\u{11BC}'..='\u{11C2}' |
        '\u{11EB}' |
        '\u{11F0}' |
        '\u{11F9}' |
        '\u{1E00}'..='\u{1E9B}' |
        '\u{1EA0}'..='\u{1EF9}' |
        '\u{1F00}'..='\u{1F15}' |
        '\u{1F18}'..='\u{1F1D}' |
        '\u{1F20}'..='\u{1F45}' |
        '\u{1F48}'..='\u{1F4D}' |
        '\u{1F50}'..='\u{1F57}' |
        '\u{1F59}' |
        '\u{1F5B}' |
        '\u{1F5D}' |
        '\u{1F5F}'..='\u{1F7D}' |
        '\u{1F80}'..='\u{1FB4}' |
        '\u{1FB6}'..='\u{1FBC}' |
        '\u{1FBE}' |
        '\u{1FC2}'..='\u{1FC4}' |
        '\u{1FC6}'..='\u{1FCC}' |
        '\u{1FD0}'..='\u{1FD3}' |
        '\u{1FD6}'..='\u{1FDB}' |
        '\u{1FE0}'..='\u{1FEC}' |
        '\u{1FF2}'..='\u{1FF4}' |
        '\u{1FF6}'..='\u{1FFC}' |
        '\u{2126}' |
        '\u{212A}'..='\u{212B}' |
        '\u{212E}' |
        '\u{2180}'..='\u{2182}' |
        '\u{3041}'..='\u{3094}' |
        '\u{30A1}'..='\u{30FA}' |
        '\u{3105}'..='\u{312C}' |
        '\u{AC00}'..='\u{D7A3}')
}

/// https://www.w3.org/TR/REC-xml/#NT-Ideographic
/// `[86] Ideographic ::= [#x4E00-#x9FA5] | #x3007 | [#x3021-#x3029]`
fn is_idiographic(c: char) -> bool {
    matches!(c, '\u{4E00}'..='\u{9FA5}' | '\u{3007}' | '\u{3021}'..='\u{3029}')
}

/// https://www.w3.org/TR/REC-xml/#NT-Letter
/// `[84] Letter ::= BaseChar | Ideographic`
fn is_letter(c: char) -> bool {
    is_base_char(c) || is_idiographic(c)
}

/// https://www.w3.org/TR/REC-xml/#NT-CombiningChar
fn is_combining_char(c: char) -> bool {
    matches!(c, '\u{0300}'..='\u{0345}' |
        '\u{0360}'..='\u{0361}' |
        '\u{0483}'..='\u{0486}' |
        '\u{0591}'..='\u{05A1}' |
        '\u{05A3}'..='\u{05B9}' |
        '\u{05BB}'..='\u{05BD}' |
        '\u{05BF}' |
        '\u{05C1}'..='\u{05C2}' |
        '\u{05C4}' |
        '\u{064B}'..='\u{0652}' |
        '\u{0670}' |
        '\u{06D6}'..='\u{06DC}' |
        '\u{06DD}'..='\u{06DF}' |
        '\u{06E0}'..='\u{06E4}' |
        '\u{06E7}'..='\u{06E8}' |
        '\u{06EA}'..='\u{06ED}' |
        '\u{0901}'..='\u{0903}' |
        '\u{093C}' |
        '\u{093E}'..='\u{094C}' |
        '\u{094D}' |
        '\u{0951}'..='\u{0954}' |
        '\u{0962}'..='\u{0963}' |
        '\u{0981}'..='\u{0983}' |
        '\u{09BC}' |
        '\u{09BE}' |
        '\u{09BF}' |
        '\u{09C0}'..='\u{09C4}' |
        '\u{09C7}'..='\u{09C8}' |
        '\u{09CB}'..='\u{09CD}' |
        '\u{09D7}' |
        '\u{09E2}'..='\u{09E3}' |
        '\u{0A02}' |
        '\u{0A3C}' |
        '\u{0A3E}' |
        '\u{0A3F}' |
        '\u{0A40}'..='\u{0A42}' |
        '\u{0A47}'..='\u{0A48}' |
        '\u{0A4B}'..='\u{0A4D}' |
        '\u{0A70}'..='\u{0A71}' |
        '\u{0A81}'..='\u{0A83}' |
        '\u{0ABC}' |
        '\u{0ABE}'..='\u{0AC5}' |
        '\u{0AC7}'..='\u{0AC9}' |
        '\u{0ACB}'..='\u{0ACD}' |
        '\u{0B01}'..='\u{0B03}' |
        '\u{0B3C}' |
        '\u{0B3E}'..='\u{0B43}' |
        '\u{0B47}'..='\u{0B48}' |
        '\u{0B4B}'..='\u{0B4D}' |
        '\u{0B56}'..='\u{0B57}' |
        '\u{0B82}'..='\u{0B83}' |
        '\u{0BBE}'..='\u{0BC2}' |
        '\u{0BC6}'..='\u{0BC8}' |
        '\u{0BCA}'..='\u{0BCD}' |
        '\u{0BD7}' |
        '\u{0C01}'..='\u{0C03}' |
        '\u{0C3E}'..='\u{0C44}' |
        '\u{0C46}'..='\u{0C48}' |
        '\u{0C4A}'..='\u{0C4D}' |
        '\u{0C55}'..='\u{0C56}' |
        '\u{0C82}'..='\u{0C83}' |
        '\u{0CBE}'..='\u{0CC4}' |
        '\u{0CC6}'..='\u{0CC8}' |
        '\u{0CCA}'..='\u{0CCD}' |
        '\u{0CD5}'..='\u{0CD6}' |
        '\u{0D02}'..='\u{0D03}' |
        '\u{0D3E}'..='\u{0D43}' |
        '\u{0D46}'..='\u{0D48}' |
        '\u{0D4A}'..='\u{0D4D}' |
        '\u{0D57}' |
        '\u{0E31}' |
        '\u{0E34}'..='\u{0E3A}' |
        '\u{0E47}'..='\u{0E4E}' |
        '\u{0EB1}' |
        '\u{0EB4}'..='\u{0EB9}' |
        '\u{0EBB}'..='\u{0EBC}' |
        '\u{0EC8}'..='\u{0ECD}' |
        '\u{0F18}'..='\u{0F19}' |
        '\u{0F35}' |
        '\u{0F37}' |
        '\u{0F39}' |
        '\u{0F3E}' |
        '\u{0F3F}' |
        '\u{0F71}'..='\u{0F84}' |
        '\u{0F86}'..='\u{0F8B}' |
        '\u{0F90}'..='\u{0F95}' |
        '\u{0F97}' |
        '\u{0F99}'..='\u{0FAD}' |
        '\u{0FB1}'..='\u{0FB7}' |
        '\u{0FB9}' |
        '\u{20D0}'..='\u{20DC}' |
        '\u{20E1}' |
        '\u{302A}'..='\u{302F}' |
        '\u{3099}' |
        '\u{309A}')
}

/// https://www.w3.org/TR/REC-xml/#NT-Extender
fn is_extender(c: char) -> bool {
    matches!(c, '\u{00B7}' | '\u{02D0}' | '\u{02D1}' | '\u{0387}' | '\u{0640}' | '\u{0E46}' |
    '\u{0EC6}' | '\u{3005}' | '\u{3031}'..='\u{3035}' | '\u{309D}'..='\u{309E}' | 
    '\u{30FC}'..='\u{30FE}')
}

/// https://www.w3.org/TR/REC-xml/#NT-Digit
fn is_digit(c: char) -> bool {
    matches!(c, '\u{0030}'..='\u{0039}' | '\u{0660}'..='\u{0669}' | '\u{06F0}'..='\u{06F9}' |
    '\u{0966}'..='\u{096F}' | '\u{09E6}'..='\u{09EF}' | '\u{0A66}'..='\u{0A6F}' |
    '\u{0AE6}'..='\u{0AEF}' | '\u{0B66}'..='\u{0B6F}' | '\u{0BE7}'..='\u{0BEF}' |
    '\u{0C66}'..='\u{0C6F}' | '\u{0CE6}'..='\u{0CEF}' | '\u{0D66}'..='\u{0D6F}' |
    '\u{0E50}'..='\u{0E59}' | '\u{0ED0}'..='\u{0ED9}' | '\u{0F20}'..='\u{0F29}')
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tests

#[test]
fn uri_test() {}
