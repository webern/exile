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
    let xtest = xml_file_list();
    for xml_path in xtest.iter() {
        let name = name_from_path(xml_path);
        let data_dir = xml_path.parent().unwrap();
        let metadata_path = metadata_filepath_from_name(&name, &data_dir);
        result.push(XmlFile {
            name,
            xml_path: xml_path.into(),
            metadata_path: metadata_path.clone(),
            metadata: load_metadata(&metadata_path),
        })
    }
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
    let metadata_file = dir
        .to_path_buf()
        .join(format!("{}{}", test_name, ".metadata.json"));
    XmlFile {
        name: test_name.to_string(),
        xml_path: xml_file,
        metadata_path: metadata_file.clone(),
        metadata: load_metadata(&metadata_file),
    }
}

fn load_metadata(p: &PathBuf) -> Metadata {
    let file = File::open(p).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn xml_file_list() -> Vec<PathBuf> {
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
        if filename.starts_with("disabled.") {
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

fn metadata_filepath_from_name<P: AsRef<Path>>(name: &str, p: P) -> PathBuf {
    p.as_ref().join(metadata_filename_from_name(name))
}
