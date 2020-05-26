use std::borrow::Cow;
use std::default::Default;
use std::io::{Cursor, Write};

use crate::error::Result;
use crate::{Element, WriteOpts};

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Version {
    None,
    One,
    OneDotOne,
}

impl Default for Version {
    fn default() -> Self {
        Version::None
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Encoding {
    None,
    Utf8,
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::None
    }
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash, Default)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Declaration {
    pub version: Version,
    pub encoding: Encoding,
}

#[derive(Debug, Clone, Eq, PartialOrd, PartialEq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
pub struct Document {
    declaration: Declaration,
    root: Element,
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
            root: Element {
                namespace: None,
                name: "root".into(),
                attributes: Default::default(),
                nodes: vec![],
            },
        }
    }
}

impl<'a> Document {
    pub fn new() -> Document {
        Document::default()
    }
}

impl Document {
    pub fn from_root(root: Element) -> Self {
        Document {
            declaration: Default::default(),
            root,
        }
    }

    pub fn root(&self) -> &Element {
        &self.root
    }

    pub fn set_root(&mut self, element_data: Element) {
        self.root = element_data;
    }

    pub fn declaration(&self) -> &Declaration {
        &self.declaration
    }

    pub fn set_declaration(&mut self, declaration: Declaration) {
        self.declaration = declaration;
    }

    pub fn write<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write,
    {
        self.write_opts(writer, &WriteOpts::default())
    }

    pub fn write_opts<W>(&self, writer: &mut W, opts: &WriteOpts) -> Result<()>
    where
        W: Write,
    {
        if self.declaration.encoding != Encoding::None || self.declaration.version != Version::None
        {
            if let Err(e) = write!(writer, "<?xml ") {
                return wrap!(e);
            }
            let mut need_space = true;
            match self.declaration.version {
                Version::None => need_space = false,
                Version::One => {
                    if let Err(e) = write!(writer, "version=\"1.0\"") {
                        return wrap!(e);
                    }
                }
                Version::OneDotOne => {
                    if let Err(e) = write!(writer, "version=\"1.1\"") {
                        return wrap!(e);
                    }
                }
            }

            match self.declaration.encoding {
                Encoding::None => {}
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
            if let Err(e) = write!(writer, "?>") {
                return wrap!(e);
            }
            if let Err(e) = opts.newline(writer) {
                return wrap!(e);
            }
        }

        if let Err(e) = self.root().write(writer, opts, 0) {
            return wrap!(e);
        }

        Ok(())
    }

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

#[macro_export]
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
</cats>
"#;

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
                version: Version::One,
                encoding: Encoding::Utf8,
            },
            root: cats_data,
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
        let expected = format!(
            r#"<root attr="&lt;&amp;&gt;&quot;🍔&quot;''">&amp;&amp;&amp;&lt;&lt;&lt;'"🍔"'&gt;&gt;&gt;&amp;&amp;&amp;</root>{}"#,
            '\n'
        );
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
