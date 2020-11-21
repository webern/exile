use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// Both attributes and elements can have a namespace alias prefix, such as `ns:foo`, where `ns` is
/// the 'prefix' and 'foo' is the 'name'. The `Name` struct provides the convenience of parsing and
/// differentiating the 'prefix' and 'name' parts.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Name {
    value: String,
}

impl Name {
    /// Instantiates a new name using `full` as the fullname. For example `new("ns:foo")` creates a
    /// new name with prefix `ns` and name `foo`. `new("foo")` creates a new name with no prefix and
    /// name `foo`, etc.
    pub(crate) fn new<S: Into<String>>(full: S) -> Self {
        Self { value: full.into() }
    }

    /// Returns the 'name' part. For example, if the `fullname` is `ns:foo`, this function returns
    /// `foo`.
    pub(crate) fn name(&self) -> &str {
        match self.find() {
            None => self.value.as_str(),
            Some(pos) => {
                let s = self.value.as_str();
                if pos == s.len() - 1 {
                    ""
                } else {
                    &s[pos + 1..]
                }
            }
        }
    }

    /// Returns the 'prefix' part. For example, if the `fullname` is `ns:foo`, this function returns
    /// `ns`.
    pub(crate) fn prefix(&self) -> Option<&str> {
        match self.find() {
            None => None,
            Some(pos) => {
                let s = self.value.as_str();
                Some(&s[..pos])
            }
        }
    }

    /// Returns both the 'prefix' and the 'name', for example `ns:foo`.
    pub(crate) fn full(&self) -> &str {
        self.value.as_str()
    }

    /// Sets the 'name' part. For example, if the value was `ns::foo`, then
    /// `set_name("bar")` would set it to `ns:bar`.
    pub(crate) fn set_name<S: AsRef<str>>(&mut self, name: S) {
        match self.prefix() {
            None => self.value = name.as_ref().into(),
            Some(prefix) => {
                let mut value = String::with_capacity(prefix.len() + 1 + name.as_ref().len());
                value.push_str(prefix);
                value.push(':');
                value.push_str(name.as_ref());
                self.value = value;
            }
        }
    }

    /// Sets the 'prefix' part. For example, if the value was `ns::foo`, then
    /// `set_prefix("xyz")` would set it to `xyz:foo`.
    pub(crate) fn set_prefix<S: AsRef<str>>(&mut self, prefix: S) {
        let name = self.name();
        let mut value = String::with_capacity(prefix.as_ref().len() + 1 + name.len());
        value.push_str(prefix.as_ref());
        value.push(':');
        value.push_str(name.as_ref());
        self.value = value;
    }

    /// Sets the complete value. For example, if the value was `ns::foo`, then
    /// `set_full("xyz:bar")` would set it to `xyz:bar`.
    pub(crate) fn set_full<S: Into<String>>(&mut self, full: S) {
        self.value = full.into();
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full())
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.full().as_ref()
    }
}

impl AsRef<String> for Name {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.value
    }
}

impl From<&str> for Name {
    fn from(s: &str) -> Self {
        Name::new(s)
    }
}

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl Name {
    fn find(&self) -> Option<usize> {
        self.value.as_str().find(':')
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn ns_foo() {
    let name = Name::new("ns:foo");
    assert_eq!("ns", name.prefix().unwrap());
    assert_eq!("foo", name.name());
    assert_eq!("ns:foo", name.full());
}

#[test]
fn no_prefix() {
    let name = Name::new("nsfoo");
    assert!(name.prefix().is_none());
    assert_eq!("nsfoo", name.name());
    assert_eq!("nsfoo", name.full());
}

#[test]
fn weird_colons() {
    let name = Name::new(":ns:foo:");
    assert_eq!("", name.prefix().unwrap());
    assert_eq!("ns:foo:", name.name());
    assert_eq!(":ns:foo:", name.full());
}

#[test]
fn no_name_test() {
    let name = Name::new("foo:");
    assert_eq!("foo", name.prefix().unwrap());
    assert_eq!("", name.name());
    assert_eq!("foo:", name.full());
}

#[test]
fn set_prefix() {
    let mut name = Name::new("xyz:bones");
    name.set_prefix("cat");
    assert_eq!("cat", name.prefix().unwrap());
    assert_eq!("bones", name.name());
    assert_eq!("cat:bones", name.full());
}

#[test]
fn set_name_test() {
    let mut name = Name::new("xyz:bones");
    name.set_name("bishop");
    assert_eq!("xyz", name.prefix().unwrap());
    assert_eq!("bishop", name.name());
    assert_eq!("xyz:bishop", name.full());
}

#[test]
fn set_full_test() {
    let mut name = Name::new("xyz:bones");
    name.set_full("cat:bishop");
    assert_eq!("cat", name.prefix().unwrap());
    assert_eq!("bishop", name.name());
    assert_eq!("cat:bishop", name.full());
}

#[test]
fn find_test() {
    assert!(Name::new("xyz").find().is_none());
    assert!(Name::new("").find().is_none());
    assert_eq!(0, Name::new(":xyz").find().unwrap());
    assert_eq!(0, Name::new(":x:yz").find().unwrap());
    assert_eq!(1, Name::new("x:yz").find().unwrap());
    assert_eq!(3, Name::new("xyz:").find().unwrap());
    assert_eq!(0, Name::new(":").find().unwrap());
}
