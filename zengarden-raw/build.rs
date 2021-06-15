use std::env;
use std::path::PathBuf;

use bindgen::EnumVariation;
use cc;
use glob::{glob, GlobError};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // watch sources
    let sources = glob("zengarden/src/*.cpp")
        .expect("Can't collect ZenGarden sources to compile.")
        .map(|item| {
            println!(
                "cargo:rerun-if-changed={}",
                item.as_ref().unwrap().to_str().unwrap()
            );
            item
        })
        .collect::<Result<Vec<PathBuf>, GlobError>>()
        .unwrap();

    compile(sources);
    link();
    generate_bindings();
}

fn compile(sources: Vec<PathBuf>) {
    let mut compiler = cc::Build::new();

    let mut builder = compiler
        .cpp(true)
        .flag("-std=c++11")
        .include("zengarden/src")
        .files(sources)
        .warnings(false);

    if cfg!(target_os = "macos") {
        env::set_var("CC", "gcc");
        env::set_var("CXX", "g++");
    }

    if cfg!(target_os = "windows") {
        env::set_var("CC", "gcc");
        env::set_var("CXX", "g++");

        builder = builder
            .target("x86_64-pc-windows-gnu")
            .host("x86_64-pc-windows-gnu");
    }

    builder.compile("zengarden");
}

fn link() {
    if let Ok(sndfile_path) = env::var("LIBSNDFILE_PATH") {
        println!("cargo:rustc-link-search={}", sndfile_path);
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-search=C:/Program Files/libsndfile/lib");
    }

    println!("cargo:rustc-link-lib=sndfile");
    println!("cargo:rustc-link-lib=pthread");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=regex");
    }
}

fn generate_bindings() {
    let mut builder = bindgen::Builder::default();
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("Can't get Cargo's $OUT_DIR."));

    builder = builder.clang_arg("-Izengarden/src");

    builder
        .header("wrapper.hpp")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .opaque_type("std::.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");
}
