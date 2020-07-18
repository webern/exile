// Automatically generate README.md and xml parse tests.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    sir_watch_alot();
    generate_readme();
    generate_tests();
}

fn generate_readme() {
    // Check for environment variable "SKIP_README". If it is set, skip README generation.
    if env::var_os("EXILE_GENERATE_README").is_none() {
        return;
    }

    let mut source = File::open("src/lib.rs").unwrap();
    let mut template = File::open("readme.template").unwrap();

    let content = cargo_readme::generate_readme(
        &PathBuf::from("."), // root
        &mut source,         // source
        Some(&mut template), // template
        // The "add x" arguments don't apply when using a template.
        true,  // add title
        false, // add badges
        false, // add license
        true,  // indent headings
    )
    .unwrap();
    let this_readme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("README.md")
        .canonicalize()
        .unwrap();
    let mut readme = File::create("README.md").unwrap();
    readme.write_all(content.as_bytes()).unwrap();
    let top_readme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("README.md")
        .canonicalize()
        .unwrap();
    if env::var_os("EXILE_GENERATE_TOP_README").is_some() {
        std::fs::copy(&this_readme_path, &top_readme_path).unwrap();
    }
}

fn generate_tests() {
    // Check for environment variable "SKIP_TEST_GENERATION". If it is set, skip test generation.
    if env::var_os("EXILE_GENERATE_TESTS").is_none() {
        println!("skipping test generation...");
        return;
    }
    println!("generating tests...");
    let test_file_path = integ_test_dir().join("parse_tests.rs");
    xtest::gen::generate_tests(test_file_path);
}

fn integ_test_dir() -> PathBuf {
    let mycrate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    mycrate_dir.join("tests")
}

fn xtest_test_dir() -> PathBuf {
    let mut mycrate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap();
    mycrate_dir.pop();
    mycrate_dir.join("xtest").canonicalize().unwrap()
}

fn exile_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .canonicalize()
        .unwrap()
}

// path walking helpers for watching files

fn list_dir(p: &Path) -> Vec<String> {
    if !p.is_dir() {
        panic!("{} is not a dir", p.display());
    }
    let mut vec = Vec::new();
    let paths = std::fs::read_dir(p).unwrap();
    for path in paths {
        vec.append(&mut list_files_recursively(&path.unwrap().path()));
    }
    vec
}

fn list_files_recursively(p: &Path) -> Vec<String> {
    let mut vec = Vec::new();
    if p.is_file() {
        if let Some(ext) = p.extension() {
            let ext = ext.to_str().unwrap();
            if ext == "rs" || ext == "json" || ext == "xml" {
                vec.push(p.canonicalize().unwrap().to_str().unwrap().to_owned());
            }
        }
    } else if p.is_dir() {
        vec.append(&mut list_dir(p));
    } else {
        panic!("unknown path type {}", p.display());
    }
    vec
}

fn sir_watch_alot() {
    let mut vec = list_files_recursively(&xtest_test_dir());
    vec.push(
        exile_dir()
            .join("src")
            .join("lib.rs")
            .to_str()
            .unwrap()
            .to_owned(),
    );
    vec.push(
        exile_dir()
            .join("readme.template")
            .to_str()
            .unwrap()
            .to_owned(),
    );
    for file in &vec {
        println!("cargo:rerun-if-changed={}", file);
    }
    println!("cargo:rerun-if-env-changed=EXILE_GENERATE_TESTS");
    println!("cargo:rerun-if-env-changed=EXILE_GENERATE_README");
}
