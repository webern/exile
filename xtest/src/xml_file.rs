use std::fs::read_to_string;
use std::path::PathBuf;

use crate::Metadata;

/// Represents a test file including paths to the test file and its metadata file.
#[derive(Debug, Clone)]
pub struct XmlFile {
    pub name: String,
    pub xml_path: PathBuf,
    pub metadata: Metadata,
    pub metadata_path: PathBuf,
    pub expected_write: Option<PathBuf>,
}

impl XmlFile {
    pub fn read_xml_file(&self) -> String {
        read_to_string(&self.xml_path).unwrap()
    }

    pub fn read_expected_write(&self) -> Option<String> {
        if let Some(p) = &self.expected_write {
            return Some(read_to_string(p).unwrap());
        }
        None
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Syntax {
    /// The XML file is expected to be parsed with success.
    Good {},
    /// The XML parser is expected to encounter an error at the given location.
    Bad {
        character_position: u64,
        line: u64,
        column: u64,
    },
}

impl Default for Syntax {
    fn default() -> Self {
        Syntax::Good {}
    }
}
