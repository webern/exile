use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::{Metadata, XmlFile};

////////////////////////////////////////////////////////////////////////////////////////////////////
// public
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn load(test_name: &str) -> XmlFile {
    load_impl(test_name, &data_dir())
}

pub fn load_all() -> Vec<XmlFile> {
    let mut result = Vec::new();
    let xtest = test_input_xml_files();
    for xml_path in xtest.iter() {
        let name = name_from_path(xml_path);
        let data_dir = xml_path.parent().unwrap();
        let metadata_path = metadata_filepath_from_name(&name, &data_dir);
        let expected_write = expected_write_filepath_from_name(&name, &data_dir);

        result.push(XmlFile {
            name,
            xml_path: xml_path.into(),
            metadata_path: metadata_path.clone(),
            metadata: load_metadata(metadata_path),
            expected_write,
        })
    }
    result.sort_by(|a, b| {
        a.name.as_str().cmp(b.name.as_str())
    });
    result
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// private
////////////////////////////////////////////////////////////////////////////////////////////////////

fn my_crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap()
}

fn load_impl(test_name: &str, dir: &PathBuf) -> XmlFile {
    let xml_file = dir.to_path_buf().join(format!("{}{}", test_name, ".xml"));
    let metadata_file = metadata_filepath_from_name(test_name, &dir);
    XmlFile {
        name: test_name.to_string(),
        xml_path: xml_file,
        metadata_path: metadata_file.clone(),
        metadata: load_metadata(metadata_file),
        expected_write: expected_write_filepath_from_name(test_name, &dir),
    }
}

fn load_metadata(p: PathBuf) -> Metadata {
    let file = File::open(p).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn test_input_xml_files() -> Vec<PathBuf> {
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
        let filename = entry.file_name();
        let filename = filename.to_string_lossy().to_string();
        if filename.starts_with("disabled.") || filename.contains(".expected.xml") {
            continue;
        }
        let ext = ext(&p);
        if ext.as_str() == "xml" {
            result.push(path);
        }
    }
    result
}

fn data_dir() -> PathBuf {
    my_crate_dir().join("data").canonicalize().unwrap()
}

fn ext(p: &PathBuf) -> String {
    p.extension().unwrap().to_string_lossy().to_string()
}

fn name_from_path(p: &PathBuf) -> String {
    p.file_name().unwrap().to_string_lossy().replace(".xml", "")
}

fn metadata_filename_from_name(name: &str) -> String {
    format!("{}{}", name, ".metadata.json")
}

fn expected_write_filename_from_name(name: &str) -> String {
    format!("{}{}", name, ".expected.xml")
}

fn metadata_filepath_from_name<P: AsRef<Path>>(name: &str, p: P) -> PathBuf {
    p.as_ref().join(metadata_filename_from_name(name))
}

fn expected_write_filepath_from_name<P: AsRef<Path>>(name: &str, p: P) -> Option<PathBuf> {
    let path = p.as_ref().join(expected_write_filename_from_name(name));
    if path.is_file() {
        Some(path)
    } else {
        None
    }
}
