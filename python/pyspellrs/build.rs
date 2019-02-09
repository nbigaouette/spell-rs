extern crate cbindgen;

use std::{env, path::Path};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config: cbindgen::Config = Default::default();
    config.language = cbindgen::Language::C;

    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or(String::from("../../target"));
    let header_file = Path::new(&target_dir).join("spell.h");

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(header_file);
}
