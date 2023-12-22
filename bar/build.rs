use std::{path::PathBuf, fs::OpenOptions, io::Write};

use molecule_codegen::{Compiler, Language};

fn compile_schema(schema: &str) {
    let out_dir = PathBuf::from("src");
    let mut compiler = Compiler::new();
    compiler
        .input_schema_file(schema)
        .generate_code(Language::Rust)
        .output_dir(out_dir)
        .run()
        .unwrap();
    println!("cargo:rerun-if-changed={}", schema);
}

fn write_schema(schema: &str, content: &[u8]) {
    let out_dir = PathBuf::from(schema);
    let mut file = OpenOptions::new().write(true).create(true).open(out_dir).unwrap();
    file.write_all(content).unwrap();
}

fn main() {
    write_schema("schemas/foo.mol", foo::SCHEMA);
    compile_schema("schemas/bar.mol");
}
