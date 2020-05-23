//! `xml-files` provides a set of XML files for testing. Each file comes with a manifest (in JSON
//! format) which describes the XML file. For example, some XML files include intentional syntax
//! errors, and the the accompanying JSON manifest will make this apparent.

use std::fs;
use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// use serde_json::Value;

pub fn data_dir() -> PathBuf {
    my_crate_dir().join("data").canonicalize().unwrap()
}

/// `TestXmlFile` Represents a test file including paths to the test file and its metadata file.
#[derive(Debug, Clone)]
pub struct TestXmlFile {
    pub name: String,
    pub xml_file: PathBuf,
    pub metadata_file: PathBuf,
    pub metadata: TestMetadata,
}

impl TestXmlFile {
    pub fn read_xml_file(&self) -> String {
        read_to_string(&self.xml_file).unwrap()
    }
}

pub fn list_test_files() -> Vec<TestXmlFile> {
    let mut result = Vec::new();
    let xml_files = list_xml_files();
    for xml_file in xml_files.iter() {
        let name = xml_file
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(".xml", "");
        let dir = xml_file.parent().unwrap();
        let metadata_file = dir
            .to_path_buf()
            .join(format!("{}{}", name, ".metadata.json"));
        result.push(TestXmlFile {
            name,
            xml_file: xml_file.into(),
            metadata_file: metadata_file.clone(),
            metadata: load_metadata(&metadata_file),
        })
    }
    result
}

pub fn get_test_info(test_name: &str) -> TestXmlFile {
    get_test_info_with_dir(test_name, &data_dir())
}

fn get_test_info_with_dir(test_name: &str, dir: &PathBuf) -> TestXmlFile {
    let xml_file = dir.to_path_buf().join(format!("{}{}", test_name, ".xml"));
    let metadata_file = dir
        .to_path_buf()
        .join(format!("{}{}", test_name, ".metadata.json"));
    TestXmlFile {
        name: test_name.to_string(),
        xml_file,
        metadata_file: metadata_file.clone(),
        metadata: load_metadata(&metadata_file),
    }
}

// pub fn open_xml_file(test_name: &str) -> File {
//     let p = data_dir().join(format!("{}.xml", test_name));
//     File::open(p).unwrap()
// }

// #[serde(rename_all = "kebab-case")]

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
// #[serde(tag = "syntax", content = "syntax_error_location")]
pub enum Syntax {
    Good {},
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

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub struct TestMetadata {
    pub description: String,
    pub syntax: Syntax,
}

// PRIVATE

fn my_crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap()
}

fn ext(p: &PathBuf) -> String {
    p.extension().unwrap().to_string_lossy().to_string()
}

fn list_all_files() -> Vec<PathBuf> {
    let mut result = Vec::new();
    let dir = data_dir();
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        // check that it's a file
        let file_type = entry.file_type().unwrap();
        if !file_type.is_file() {
            continue;
        }

        let p = entry.path();
        let ext = ext(&p);
        if ext.as_str() == "json" || ext.as_str() == "xml" {
            result.push(path);
        }
    }
    result
}

fn list_xml_files() -> Vec<PathBuf> {
    list_all_files()
        .into_iter()
        .filter(|p| ext(&p).as_str() == "xml")
        .collect()
}

// fn list_json_files() -> Vec<PathBuf> {
//     list_all_files()
//         .into_iter()
//         .filter(|p| ext(&p).as_str() == "json")
//         .collect()
// }

fn load_metadata(p: &PathBuf) -> TestMetadata {
    // Open the file in read-only mode with buffer.
    let file = File::open(p).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}
