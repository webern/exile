// use std::collections::HashMap;
// use std::ffi::OsStr;
// use std::fs;
// use std::path::{Path, PathBuf};
//
// use test_dir;

// fn get_files() -> Vec<PathBuf> {
//     let mut result = Vec::new();
//     let dir = test_dir::xml_data_dir();
//     let entries = fs::read_dir(dir).unwrap();
//     for entry in entries {
//         let entry = entry.unwrap();
//         let path = entry.path();
//
//         // check that it's a file
//         let file_type = entry.file_type().unwrap();
//         if !file_type.is_file() {
//             continue;
//         }
//
//         result.push(path);
//     }
//     result
// }

// #[derive(Debug, Clone)]
// pub struct TestXmlFile {
//     pub name: String,
//     pub xml: PathBuf,
//     pub metadata: PathBuf,
// }

// pub fn get_xtest(listing: &Vec<PathBuf>) -> Vec<TestXmlFile> {
//     let mut map: HashMap<String, TestXmlFile> = HashMap::new();
//     let mut result = Vec::new();
//     for p in listing.iter() {
//         let fname = get_filename(p);
//         let ext = get_ext(p);
//         let mut test_name = "".to_string();
//         let mut is_xml = true;
//         match ext.as_str() {
//             "xml" => {
//                 test_name = fname.replace(".xml", "");
//             }
//             "json" => {
//                 is_xml = false;
//                 let sub = fname.replace(".json", "");
//                 let metadata_str = PathBuf::from(&sub)
//                     .extension()
//                     .unwrap()
//                     .to_str()
//                     .unwrap()
//                     .to_string();
//                 assert!(metadata_str == "metadata".to_string());
//                 test_name = sub.replace(".metadata", "")
//             }
//             _ => panic!("could not parse the path '{}'", p.display()),
//         }
//         let mut txf = TestXmlFile {
//             name: Default::default(),
//             xml: Default::default(),
//             metadata: Default::default(),
//         };
//         if let Some(found) = map.get_mut(&test_name) {
//             txf = found.clone();
//         }
//         txf.name = test_name.clone();
//         if is_xml {
//             txf.xml = p.clone();
//         } else {
//             txf.metadata = p.clone();
//         }
//         map.insert(test_name, txf);
//     }
//     for entry in map.iter() {
//         result.push(entry.1.clone())
//     }
//     result
// }

// fn get_ext<P: AsRef<Path>>(p: P) -> String {
//     Path::new(p.as_ref())
//         .extension()
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string()
// }
//
// fn get_filename<P: AsRef<Path>>(p: P) -> String {
//     Path::new(p.as_ref())
//         .file_name()
//         .unwrap()
//         .to_str()
//         .unwrap()
//         .to_string()
// }

// #[test]
// fn init_logger() {
//     let _ = env_logger::builder().is_test(true).try_init();
// }

// #[test]
// fn test() {
//     init_logger();
//     let files = get_files();
//     let stuff = get_xtest(&files);
//     assert!(stuff.len() > 0);
//     for x in stuff.iter() {
//         println!("{:?}", x);
//         let s = std::fs::read_to_string(&x.xml).unwrap();
//         let result = exile::parse_str(s.as_str());
//         //assert!(result.is_err());
//     }
// }
