use std::path::PathBuf;

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

fn main() {
    compile_schema("schemas/foo.mol");
}
