use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;
use cc;
use glob::{glob, GlobError};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");
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
        .flag("-std=c++11")
        .include("zengarden/src")
        .files(sources)
        .warnings(false);

    if cfg!(macos) {
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    if cfg!(windows) {
        env::set_var("CC", "gcc");
        env::set_var("CXX", "g++");

        builder = builder
            .target("x86_64-pc-windows-gnu")
            .host("x86_64-pc-windows-gnu");

        println!("cargo:rustc-link-search=C:/Program Files/libsndfile/lib");
    }

    builder.compile("zengarden");

    if let Ok(sndfile_path) = env::var("LIBSNDFILE_PATH") {
        println!("cargo:rustc-link-search={}", sndfile_path);
    }

    println!("cargo:rustc-link-lib=sndfile");
    println!("cargo:rustc-link-lib=regex");
    println!("cargo:rustc-link-lib=pthread");
}

fn generate_bindings() {
    let mut builder = bindgen::Builder::default();
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Can't get Cargo's $OUT_DIR."));

    builder = builder
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-Izengarden/src")
        .clang_arg("-std=c++11");

    if cfg!(windows) {
        builder = builder.clang_arg("--target=x86_64-pc-windows-gnu");
    }

    builder
        .header("wrapper.hpp")
        .default_enum_style(EnumVariation::Rust { non_exhaustive: false })
        .opaque_type("std::.*")
        // .opaque_type("size_type")
        // .opaque_type("std_value")
        .opaque_type("__gnu_cxx___min")
        .opaque_type("__gnu_cxx___max")
        .opaque_type("_Value")
        .opaque_type("_Tp")
        .blacklist_item("std_value")
        // .blacklist_item("__gnu_cxx___min")
        // .blacklist_item("__gnu_cxx___max")
        // .blacklist_type("_Value")
        // .blacklist_type("_Tp")
        .blacklist_type("size_type")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");
}
