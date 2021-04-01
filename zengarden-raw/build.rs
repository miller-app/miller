use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;
use cc;
use glob::{glob, GlobError};

fn main() {
    // compile();
    generate_bindings();
}

fn compile() {
    let sources = glob("zengarden/src/*.cpp")
        .expect("Can't collect ZenGarden sources to compile.")
        .collect::<Result<Vec<PathBuf>, GlobError>>()
        .unwrap();

    let mut compiler = cc::Build::new();

    let mut builder = compiler
        .cpp(true)
        .include("zengarden/src")
        .files(sources)
        .warnings(false)
        .flag("-lregex")
        .flag("-lpthread")
        .flag("-std=c++11");

    if cfg!(macos) {
        builder = builder.flag("-framework Accelerate")
    }

    if cfg!(windows) {
        env::set_var("CC", "gcc");
        env::set_var("CXX", "g++");

        builder = builder
            .target("x86_64-pc-windows-gnu")
            .host("x86_64-pc-windows-gnu");
    }

    builder.compile("zengarden");
}

fn generate_bindings() {
    let mut builder = bindgen::Builder::default();
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Can't get Cargo's $OUT_DIR."));

    builder = builder.clang_arg("-Izengarden/src").clang_arg("-std=c++11");

    if cfg!(windows) {
        builder = builder.clang_arg("--target=x86_64-pc-windows-gnu");
    }

    builder
        .opaque_type("std::.*")
        .header("wrapper.hpp")
        .default_enum_style(EnumVariation::NewType { is_bitfield: false })
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");
}
