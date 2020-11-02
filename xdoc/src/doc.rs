use std::borrow::Cow;
use std::default::Default;
use std::io::{Cursor, Write};

use crate::error::Result;
use crate::{Element, Misc, WriteOpts};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
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

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
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

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
/// The XML declaration at the start of the XML Document.
pub struct Declaration {
    /// The version of the XML Document. Default is `Version::V10` when `None`.
    pub version: Option<Version>,
    /// The encoding of the XML Document. Default is `Encoding::Utf8` when `None`.
    pub encoding: Option<Encoding>,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
/// Represents an XML Document.
pub struct Document {
    declaration: Declaration,
    // TODO - add doctype support https://github.com/webern/exile/issues/22
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    doctypedecl: Option<()>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(feature = "serde", serde(default))]
    prolog_misc: Vec<Misc>,
    root: Element,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(feature = "serde", serde(default))]
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
            if let Err(e) = write!(writer, "<?xml ") {
                return wrap!(e);
            }
            let need_space = true;
            if let Some(version) = &self.declaration.version {
                match version {
                    Version::V10 => {
                        if let Err(e) = write!(writer, "version=\"1.0\"") {
                            return wrap!(e);
                        }
                    }
                    Version::V11 => {
                        if let Err(e) = write!(writer, "version=\"1.1\"") {
                            return wrap!(e);
                        }
                    }
                }
            }
            if let Some(encoding) = &self.declaration.encoding {
                match encoding {
                    Encoding::Utf8 => {
                        if need_space {
                            if let Err(e) = write!(writer, " ") {
                                return wrap!(e);
                            }
                        }
                        if let Err(e) = write!(writer, "encoding=\"UTF-8\"") {
                            return wrap!(e);
                        }
                    }
                }
            }
            if let Err(e) = write!(writer, "?>") {
                return wrap!(e);
            }
            if let Err(e) = opts.newline(writer) {
                return wrap!(e);
            }
        }
        for misc in self.prolog_misc() {
            if let Err(e) = misc.write(writer, opts, 0) {
                return wrap!(e);
            }
            if let Err(e) = opts.newline(writer) {
                return wrap!(e);
            }
        }
        if let Err(e) = self.root().write(writer, opts, 0) {
            return wrap!(e);
        }
        for misc in self.epilog_misc() {
            if let Err(e) = opts.newline(writer) {
                return wrap!(e);
            }
            if let Err(e) = misc.write(writer, opts, 0) {
                return wrap!(e);
            }
        }

        Ok(())
    }

    /// Write the `Document` to a `String` using the given options.
    pub fn to_string_opts(&self, opts: &WriteOpts) -> Result<String> {
        let mut c = Cursor::new(Vec::new());
        if let Err(e) = self.write_opts(&mut c, &opts) {
            return wrap!(e);
        }
        let data = c.into_inner();
        match std::str::from_utf8(data.as_slice()) {
            Ok(s) => Ok(s.to_owned()),
            Err(e) => wrap!(e),
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

    use crate::doc::{Declaration, Encoding, Version};
    use crate::*;

    fn assert_ezfile(doc: &Document) {
        let root = doc.root();
        let root_data = root;
        assert_eq!(root_data.name, "cats");
        assert_eq!(root_data.namespace, None);
        assert_eq!(root_data.attributes.map().len(), 0);
        assert_eq!(root_data.nodes.len(), 2);
        let bones_element = root_data.nodes.get(0).unwrap();
        if let Node::Element(bones) = bones_element {
            assert_eq!(bones.name, "cat");
            assert_eq!(bones.namespace, None);
            assert_eq!(bones.attributes.map().len(), 1);
            assert_eq!(bones.nodes.len(), 0);
            let name = bones.attributes.map().get("name").unwrap();
            assert_eq!(name, "bones");
        } else {
            panic!("bones was supposed to be an element but was not");
        }
        let bishop_element = root_data.nodes.get(1).unwrap();
        if let Node::Element(bishop) = bishop_element {
            assert_eq!(bishop.name, "cat");
            assert_eq!(bishop.namespace, None);
            assert_eq!(bishop.attributes.map().len(), 1);
            let name = bishop.attributes.map().get("name").unwrap();
            assert_eq!(name, "bishop");
            // assert text data
            assert_eq!(bishop.nodes.len(), 1);
            if let Node::Text(text) = bishop.nodes.get(0).unwrap() {
                assert_eq!(text, "punks");
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
        let bones_data = Element {
            namespace: None,
            name: "cat".into(),
            attributes: OrdMap::from(map! { "name".to_string() => "bones".to_string() }),
            nodes: Vec::default(),
        };

        let bishop_data = Element {
            namespace: None,
            name: "cat".into(),
            attributes: OrdMap::from(map! { "name".to_string() => "bishop".to_string() }),
            nodes: vec![Node::Text("punks".to_string())],
        };

        let bones_element = Node::Element(bones_data);
        let bishop_element = Node::Element(bishop_data);

        let cats_data = Element {
            namespace: None,
            name: "cats".into(),
            attributes: Default::default(),
            nodes: vec![bones_element, bishop_element],
        };

        Document {
            declaration: Declaration {
                version: Some(Version::V10),
                encoding: Some(Encoding::Utf8),
            },
            doctypedecl: None,
            prolog_misc: vec![],
            root: cats_data,
            epilog_misc: vec![],
        }
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
        assert_eq!(data_str, EZFILE_STR);
    }

    #[test]
    fn test_escapes() {
        let expected = r#"<root attr="&lt;&amp;&gt;&quot;🍔&quot;''">&amp;&amp;&amp;&lt;&lt;&lt;'"🍔"'&gt;&gt;&gt;&amp;&amp;&amp;</root>"#;
        let mut root = Element::default();
        root.name = "root".into();
        root.attributes
            .mut_map()
            .insert("attr".into(), "<&>\"🍔\"\'\'".into());
        root.nodes.push(Node::Text("&&&<<<'\"🍔\"'>>>&&&".into()));
        let doc = Document::from_root(root);

        let mut c = Cursor::new(Vec::new());
        let result = doc.write(&mut c);
        assert!(result.is_ok());
        let data = c.into_inner();
        let data_str = std::str::from_utf8(data.as_slice()).unwrap();
        assert_eq!(expected, data_str);
    }
}
