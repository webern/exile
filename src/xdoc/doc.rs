use std::borrow::Cow;
use std::default::Default;
use std::fmt::{Display, Formatter};
use std::io::{Cursor, Write};
use std::path::Path;

use crate::error::OtherError;
use crate::xdoc::error::Result;
use crate::{Element, Index, Misc, Pi, WriteOpts};

#[derive(Debug, Default, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash)]
/// Represents the XML Version being used.
pub enum Version {
    /// The XML Version is 1.0.
    #[default]
    V10,
    /// The XML Version is 1.1.
    V11,
}

#[derive(Debug, Default, Clone, Copy, Eq, Ord, PartialOrd, PartialEq, Hash)]
/// The encoding of the XML Document, currently only UTF-8 is supported.
pub enum Encoding {
    /// The encoding is UTF-8.
    #[default]
    Utf8,
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

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
/// Represents an XML Document.
pub struct Document {
    prolog: Prolog,
    root: Element,
    epilog_misc: Vec<Misc>,
}

impl Unpin for Document {}

/// https://www.w3.org/TR/xml/#NT-prolog
/// ```text
/// [22] prolog ::= XMLDecl? Misc* (doctypedecl Misc*)?
/// ```
#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
struct Prolog {
    // TODO xml_decl should be an Option and inside it, Version should not be optional.
    xml_decl: Declaration,
    misc_before_doctype: Vec<Misc>,
    doctypedecl: Option<String>,
    misc_after_doctype: Vec<Misc>,
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

impl Document {
    /// Create a new default document.
    pub fn new() -> Document {
        Document::default()
    }
}

impl Document {
    /// Construct a new `Document` using the given `Element` as the root.
    pub fn from_root(root: Element) -> Self {
        Document {
            prolog: Default::default(),
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
        &self.prolog.xml_decl
    }

    /// Set the `Declaration` object for the `Document`.
    pub fn set_declaration(&mut self, declaration: Declaration) {
        self.prolog.xml_decl = declaration;
    }

    /// Set the doctype declaration. This will go before any prolog comments or processing
    /// instructions you have added with [`add_prolog_comment`] or [`add_prolog_pi`]. Once this has
    /// been set, using those functions will place additional comments or processing instructions
    /// *after* the doctype declaration.
    #[cfg(feature = "doctype_wip")]
    pub fn set_doctype<S: Into<String>>(&mut self, doctype: S) -> Result<()> {
        self.prolog.doctypedecl = Some(doctype.into());
        Ok(())
    }

    /// Disabled doctype setter.
    #[cfg(not(feature = "doctype_wip"))]
    pub fn set_doctype<S: Into<String>>(&mut self, _: S) -> Result<()> {
        Ok(())
    }

    /// Add a comment before the document root element.
    pub fn add_prolog_comment<S: Into<String>>(&mut self, comment: S) -> Result<()> {
        // TODO check for --
        if self.prolog.doctypedecl.is_none() {
            self.prolog
                .misc_before_doctype
                .push(Misc::Comment(comment.into()));
        } else {
            self.prolog
                .misc_after_doctype
                .push(Misc::Comment(comment.into()));
        }
        Ok(())
    }

    /// Add a processing instruction before the document root element.
    pub fn add_prolog_pi(&mut self, pi: Pi) {
        if self.prolog.doctypedecl.is_none() {
            self.prolog.misc_before_doctype.push(Misc::Pi(pi));
        } else {
            self.prolog.misc_after_doctype.push(Misc::Pi(pi));
        }
    }

    /// Remove all [`PI`] and comment entries before the root element.
    pub fn clear_prolog_misc(&mut self) {
        self.prolog.misc_before_doctype.clear();
        self.prolog.misc_after_doctype.clear();
    }

    /// Access the `Misc` entries before the root element.
    pub fn prolog_misc(&self) -> impl Iterator<Item = &Misc> + '_ {
        self.prolog
            .misc_before_doctype
            .iter()
            .chain(self.prolog.misc_after_doctype.iter())
    }

    /// Add a comment after the document root element.
    pub fn add_epilog_comment<S: Into<String>>(&mut self, comment: S) -> Result<()> {
        // TODO check for --
        self.epilog_misc.push(Misc::Comment(comment.into()));
        Ok(())
    }

    /// Add a processing instruction after the document root element.
    pub fn add_epilog_pi(&mut self, pi: Pi) {
        self.epilog_misc.push(Misc::Pi(pi));
    }

    /// Clear all `Misc` entries after the root element.
    pub fn clear_epilog_misc(&mut self) {
        self.epilog_misc.clear()
    }

    /// Access the `Misc` entries after the root element.
    pub fn epilog_misc(&self) -> std::slice::Iter<'_, Misc> {
        self.epilog_misc.iter()
    }

    /// Write the `Document` to the `Write` object.
    pub fn write<W>(&self, writer: &mut W) -> crate::error::Result<()>
    where
        W: Write,
    {
        self.write_opts(writer, &WriteOpts::default())
            .map_err(crate::error::Error::XdocErr)
    }

    /// Write the `Document` to the `Write` object using the given options.
    pub fn write_opts<W>(&self, writer: &mut W, opts: &WriteOpts) -> Result<()>
    where
        W: Write,
    {
        if self.declaration().encoding.is_some() || self.declaration().version.is_some() {
            xwrite!(writer, "<?xml ")?;
            let need_space = true;
            if let Some(version) = &self.declaration().version {
                match version {
                    Version::V10 => {
                        xwrite!(writer, "version=\"1.0\"")?;
                    }
                    Version::V11 => {
                        xwrite!(writer, "version=\"1.1\"")?;
                    }
                }
            }
            if let Some(encoding) = &self.declaration().encoding {
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
        for misc in &self.prolog.misc_before_doctype {
            misc.write(writer, opts, 0)?;
            opts.newline(writer)?;
        }
        if let Some(doctype) = &self.prolog.doctypedecl {
            xwrite!(writer, "{}", doctype)?;
            opts.newline(writer)?;
        }
        for misc in &self.prolog.misc_after_doctype {
            misc.write(writer, opts, 0)?;
            opts.newline(writer)?;
        }
        self.root().write(writer, opts, 0)?;
        for misc in self.epilog_misc() {
            opts.newline(writer)?;
            misc.write(writer, opts, 0)?;
        }
        opts.newline(writer)?;
        Ok(())
    }

    /// Write the `Document` to a `String` using the given options.
    pub fn to_string_opts(&self, opts: &WriteOpts) -> Result<String> {
        let mut c = Cursor::new(Vec::new());
        self.write_opts(&mut c, opts)?;
        let data = c.into_inner();
        match std::str::from_utf8(data.as_slice()) {
            Ok(s) => Ok(s.to_owned()),
            Err(e) => wrap_err!(e),
        }
    }

    /// Save a document to a file.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> crate::error::Result<()> {
        std::fs::write(path.as_ref(), self.to_string().as_bytes()).map_err(|e| {
            crate::error::Error::Other(OtherError {
                throw_site: throw_site!(),
                message: Some(format!("Unable to save file '{}'", path.as_ref().display())),
                source: Some(Box::new(e)),
            })
        })
    }

    /// Create an index of the elements in this document. Takes ownership of `document`.
    pub fn index(self) -> Index {
        Index::build(self)
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let opts = WriteOpts::default();
        let s = self
            .to_string_opts(&opts)
            .unwrap_or_else(|_| "<error/>".to_string());
        write!(f, "{s}")
    }
}

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
</cats>
"#;

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
        let mut expected = r#"<root attr="&lt;&amp;&gt;&quot;🍔&quot;''">&amp;&amp;&amp;&lt;&lt;&lt;'"🍔"'&gt;&gt;&gt;&amp;&amp;&amp;</root>"#.to_owned();
        expected.push('\n');
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
