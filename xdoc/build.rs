// Automatically generate README.md

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    generate_readme();
}

fn generate_readme() {
    println!("cargo:rerun-if-env-changed=EXILE_GENERATE_README");
    println!("cargo:rerun-if-changed=readme.template");
    println!("cargo:rerun-if-changed=src/lib.rs");
    // Check for environment variable "SKIP_README". If it is set, skip README generation.
    if env::var_os("XDOC_GENERATE_README").is_none() {
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
    let mut readme = File::create("README.md").unwrap();
    readme.write_all(content.as_bytes()).unwrap();
}
