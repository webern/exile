use std::borrow::Cow;
use std::default::Default;
use std::io::{Cursor, Write};

use crate::xdoc::error::Result;
use crate::{Element, Misc, WriteOpts};

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash)]
/// Represents the XML Version being used.
pub enum Version {
    /// The XML Version is 1.0.
    V10,
    /// The XML Version is 1.1.
    V11,
}

impl Default for Version {
    fn default() -> Self {
        Version::V10
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash)]
/// The encoding of the XML Document, currently only UTF-8 is supported.
pub enum Encoding {
    /// The encoding is UTF-8.
    Utf8,
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Utf8
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
/// The XML declaration at the start of the XML Document.
pub struct Declaration {
    /// The version of the XML Document. `None` is the same as `Version::V10` except that it is not
    /// printed in the XML document. That is, XML defaults to 1.0 in the absence of a declaration.
    pub version: Option<Version>,
    /// The encoding of the XML Document. `None` is the same as `Encoding::Utf8` except that it is
    /// not printed in the XML document. That is, XML defaults to `UTF-8` in the absence of a
    /// declaration.
    pub encoding: Option<Encoding>,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
/// Represents an XML Document.
pub struct Document {
    declaration: Declaration,
    // TODO - add doctype support https://github.com/webern/exile/issues/22
    doctypedecl: Option<()>,
    prolog_misc: Vec<Misc>,
    root: Element,
    epilog_misc: Vec<Misc>,
}

impl<'a> From<&'a Element> for Cow<'a, Element> {
    fn from(x: &'a Element) -> Cow<'a, Element> {
        Cow::Borrowed(x)
    }
}

impl<'a> From<Element> for Cow<'a, Element> {
    fn from(x: Element) -> Cow<'a, Element> {
        Cow::Owned(x)
    }
}

impl Default for Document {
    fn default() -> Self {
        Document {
            declaration: Declaration::default(),
            doctypedecl: None,
            prolog_misc: Vec::new(),
            root: Element::default(),
            epilog_misc: Vec::new(),
        }
    }
}

impl<'a> Document {
    /// Create a new default document.
    pub fn new() -> Document {
        Document::default()
    }
}

impl Document {
    /// Construct a new `Document` using the given `Element` as the root.
    pub fn from_root(root: Element) -> Self {
        Document {
            declaration: Default::default(),
            doctypedecl: None,
            prolog_misc: Vec::new(),
            root,
            epilog_misc: Vec::new(),
        }
    }

    /// Get the root `Element`.
    pub fn root(&self) -> &Element {
        &self.root
    }

    /// Set the root `Element`.
    pub fn set_root(&mut self, element_data: Element) {
        self.root = element_data;
    }

    /// Get a mutable reference to the root `Element`.
    pub fn root_mut(&mut self) -> &mut Element {
        &mut self.root
    }

    /// Get the `Declaration` object.
    pub fn declaration(&self) -> &Declaration {
        &self.declaration
    }

    /// Set the `Declaration` object for the `Document`.
    pub fn set_declaration(&mut self, declaration: Declaration) {
        self.declaration = declaration;
    }

    /// Add a `Misc` before the root element.
    pub fn push_prolog_misc(&mut self, misc: Misc) {
        self.prolog_misc.push(misc)
    }

    /// Clear all `Misc` entries before the root element.
    pub fn clear_prolog_misc(&mut self) {
        self.prolog_misc.clear()
    }

    /// Access the `Misc` entries after the root element.
    pub fn epilog_misc(&self) -> std::slice::Iter<'_, Misc> {
        self.epilog_misc.iter()
    }

    /// Add a `Misc` after the root element.
    pub fn push_epilog_misc(&mut self, misc: Misc) {
        self.epilog_misc.push(misc)
    }

    /// Clear all `Misc` entries after the root element.
    pub fn clear_epilog_misc(&mut self) {
        self.epilog_misc.clear()
    }

    /// Access the `Misc` entries before the root element.
    pub fn prolog_misc(&self) -> std::slice::Iter<'_, Misc> {
        self.prolog_misc.iter()
    }

    /// Write the `Document` to the `Write` object.
    pub fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.write_opts(writer, &WriteOpts::default())
    }

    /// Write the `Document` to the `Write` object using the given options.
    pub fn write_opts<W>(&self, writer: &mut W, opts: &WriteOpts) -> Result<()>
    where
        W: Write,
    {
        if self.declaration.encoding.is_some() || self.declaration.version.is_some() {
            xwrite!(writer, "<?xml ")?;
            let need_space = true;
            if let Some(version) = &self.declaration.version {
                match version {
                    Version::V10 => {
                        xwrite!(writer, "version=\"1.0\"")?;
                    }
                    Version::V11 => {
                        xwrite!(writer, "version=\"1.1\"")?;
                    }
                }
            }
            if let Some(encoding) = &self.declaration.encoding {
                match encoding {
                    Encoding::Utf8 => {
                        if need_space {
                            xwrite!(writer, " ")?;
                        }
                        xwrite!(writer, "encoding=\"UTF-8\"")?
                    }
                }
            }
            xwrite!(writer, "?>")?;
            if let Err(e) = opts.newline(writer) {
                return wrap_err!(e);
            }
        }
        for misc in self.prolog_misc() {
            misc.write(writer, opts, 0)?;
            opts.newline(writer)?;
        }
        self.root().write(writer, opts, 0)?;
        for misc in self.epilog_misc() {
            opts.newline(writer)?;
            misc.write(writer, opts, 0)?;
        }
        Ok(())
    }

    /// Write the `Document` to a `String` using the given options.
    pub fn to_string_opts(&self, opts: &WriteOpts) -> Result<String> {
        let mut c = Cursor::new(Vec::new());
        self.write_opts(&mut c, &opts)?;
        let data = c.into_inner();
        match std::str::from_utf8(data.as_slice()) {
            Ok(s) => Ok(s.to_owned()),
            Err(e) => wrap_err!(e),
        }
    }
}

impl ToString for Document {
    fn to_string(&self) -> String {
        let opts = WriteOpts::default();
        match self.to_string_opts(&opts) {
            Ok(s) => s,
            Err(_) => "<error/>".to_string(),
        }
    }
}

// a macro for creating a btree map, kind of like vec!
#[allow(unused_macros)]
macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::BTreeMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::Node;

    use super::*;

    fn assert_ezfile(doc: &Document) {
        let root = doc.root();
        let root_data = root;
        assert_eq!("cats", root_data.name());
        assert_eq!(None, root_data.prefix());
        assert_eq!(0, root_data.attributes_len());
        assert_eq!(2, root_data.nodes_len());
        let bones_element = root_data.node(0).unwrap();
        if let Node::Element(bones) = bones_element {
            assert_eq!("cat", bones.name());
            assert_eq!(None, bones.prefix());
            assert_eq!(1, bones.attributes_len());
            assert_eq!(0, bones.nodes_len());
            let name_attribute_value = bones.attribute("name").unwrap();
            assert_eq!("bones", name_attribute_value);
        } else {
            panic!("bones was supposed to be an element but was not");
        }
        let bishop_element = root_data.node(1).unwrap();
        if let Node::Element(bishop) = bishop_element {
            assert_eq!("cat", bishop.name());
            assert_eq!(None, bishop.prefix());
            assert_eq!(1, bishop.attributes_len());
            let name_attribute_value = bishop.attribute("name").unwrap();
            assert_eq!("bishop", name_attribute_value);
            // assert text data
            assert_eq!(1, bishop.nodes_len());
            if let Node::Text(text) = bishop.node(0).unwrap() {
                assert_eq!("punks", text);
            } else {
                panic!("Expected to find a text node but it was not there.");
            }
        } else {
            panic!("bones was supposed to be an element but was not");
        }
    }

    const EZFILE_STR: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<cats>
  <cat name="bones"/>
  <cat name="bishop">punks</cat>
</cats>"#;

    fn create_ezfile() -> Document {
        let mut bones = Element::from_name("cat");
        bones.add_attribute("name", "bones");
        let mut bishop = Element::from_name("cat");
        bishop.add_attribute("name", "bishop");
        bishop.add_text("punks");
        let mut cats = Element::from_name("cats");
        cats.add_child(bones);
        cats.add_child(bishop);
        let mut doc = Document::from_root(cats);
        doc.set_declaration(Declaration {
            version: Some(Version::V10),
            encoding: Some(Encoding::Utf8),
        });
        doc
    }

    #[test]
    fn test_ezfile_create() {
        let ezfile = create_ezfile();
        assert_ezfile(&ezfile);
    }

    #[test]
    fn test_ezfile_to_string() {
        let doc = create_ezfile();
        let mut c = Cursor::new(Vec::new());
        let result = doc.write(&mut c);
        assert!(result.is_ok());
        let data = c.into_inner();
        let data_str = std::str::from_utf8(data.as_slice()).unwrap();
        assert_eq!(EZFILE_STR, data_str);
    }

    #[test]
    fn test_escapes() {
        let expected = r#"<root attr="&lt;&amp;&gt;&quot;🍔&quot;''">&amp;&amp;&amp;&lt;&lt;&lt;'"🍔"'&gt;&gt;&gt;&amp;&amp;&amp;</root>"#;
        let mut root = Element::default();
        root.set_name("root");
        root.add_attribute("attr", "<&>\"🍔\"\'\'");
        root.add_text("&&&<<<'\"🍔\"'>>>&&&");
        let doc = Document::from_root(root);

        let mut c = Cursor::new(Vec::new());
        let result = doc.write(&mut c);
        assert!(result.is_ok());
        let data = c.into_inner();
        let data_str = std::str::from_utf8(data.as_slice()).unwrap();
        assert_eq!(expected, data_str);
    }
}
