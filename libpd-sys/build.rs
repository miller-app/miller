use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

use bindgen;
use cc;
use glob::{glob, GlobError};

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    compile();

    generate_bindings();
}

fn compile() {
    let pd_sources = glob("libpd/pure-data/src/*.c")
        .expect("Can't collect Pd sources to compile.")
        .collect::<Result<Vec<PathBuf>, GlobError>>()
        .unwrap();
    let libpd_sources = glob("libpd/libpd_wrapper/**/*.c")
        .expect("Can't collect libpd sources to compile.")
        .collect::<Result<Vec<PathBuf>, GlobError>>()
        .unwrap();

    let mut compiler = cc::Build::new();

    compiler
        .include("libpd/pure-data/src")
        .include("libpd/libpd_wrapper")
        .files(pd_sources)
        .files(libpd_sources)
        .file("libpd/pure-data/extra/bob~/bob~.c")
        .file("libpd/pure-data/extra/bonk~/bonk~.c")
        .file("libpd/pure-data/extra/choice/choice.c")
        .file("libpd/pure-data/extra/fiddle~/fiddle~.c")
        .file("libpd/pure-data/extra/loop~/loop~.c")
        .file("libpd/pure-data/extra/lrshift~/lrshift~.c")
        .file("libpd/pure-data/extra/pique/pique.c")
        .file("libpd/pure-data/extra/pd~/pdsched.c")
        .file("libpd/pure-data/extra/pd~/pd~.c")
        .file("libpd/pure-data/extra/sigmund~/sigmund~.c")
        .file("libpd/pure-data/extra/stdout/stdout.c")
        .define("PD", None)
        .define("USEAPI_DUMMY", None)
        .warnings(false);

    if cfg!(windows) {
        env::set_var("CC", "gcc");
        env::set_var("CXX", "g++");
        env::set_var("WINDRES", "windres");

        compiler
            .target("x86_64-pc-windows-gnu")
            .host("x86_64-pc-windows-gnu")
            .define("PD_INTERNAL", None)
            .define("PD_LONGINTTYPE", Some("long long"));
    // .define("MSW", None)
    // .define("NT", None)
    // .define("WIN32", None)
    // .define("_WIN32", None)
    // .define("WINDOWS", None)
    // .define("_WINDOWS", None);
    } else {
        compiler
            .define("HAVE_LIBDL", None)
            .define("HAVE_UNISTD_H", None);
    }

    compiler.compile("pd");
}

fn generate_bindings() {
    let bindgen = bindgen::Builder::default();

    // https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-450750547
    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );

    let bindgen = bindgen
        .clang_arg("-Ilibpd/pure-data/src")
        .clang_arg("-Ilibpd/libpd_wrapper")
        .clang_arg("-DPD")
        .clang_arg("-DUSEAPI_DUMMY")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ignored_macros));

    let bindgen = if cfg!(not(windows)) {
        bindgen
            .clang_arg("-DHAVE_LIBDL")
            .clang_arg("-DHAVE_UNISTD_H")
    } else {
        bindgen
            .blacklist_type("LPMONITORINFOEXA?W?")
            .blacklist_type("LPTOP_LEVEL_EXCEPTION_FILTER")
            .blacklist_type("MONITORINFOEXA?W?")
            .blacklist_type("PEXCEPTION_FILTER")
            .blacklist_type("PEXCEPTION_ROUTINE")
            .blacklist_type("PSLIST_HEADER")
            .blacklist_type("PTOP_LEVEL_EXCEPTION_FILTER")
            .blacklist_type("PVECTORED_EXCEPTION_HANDLER")
            .blacklist_type("_?L?P?CONTEXT")
            .blacklist_type("_?L?P?EXCEPTION_POINTERS")
            .blacklist_type("_?P?DISPATCHER_CONTEXT")
            .blacklist_type("_?P?EXCEPTION_REGISTRATION_RECORD")
            .blacklist_type("_?P?IMAGE_TLS_DIRECTORY.*")
            .blacklist_type("_?P?NT_TIB")
            .blacklist_type("tagMONITORINFOEXA")
            .blacklist_type("tagMONITORINFOEXW")
            .blacklist_function("AddVectoredContinueHandler")
            .blacklist_function("AddVectoredExceptionHandler")
            .blacklist_function("CopyContext")
            .blacklist_function("GetThreadContext")
            .blacklist_function("GetXStateFeaturesMask")
            .blacklist_function("InitializeContext")
            .blacklist_function("InitializeContext2")
            .blacklist_function("InitializeSListHead")
            .blacklist_function("InterlockedFlushSList")
            .blacklist_function("InterlockedPopEntrySList")
            .blacklist_function("InterlockedPushEntrySList")
            .blacklist_function("InterlockedPushListSListEx")
            .blacklist_function("LocateXStateFeature")
            .blacklist_function("QueryDepthSList")
            .blacklist_function("RaiseFailFastException")
            .blacklist_function("RtlCaptureContext")
            .blacklist_function("RtlCaptureContext2")
            .blacklist_function("RtlFirstEntrySList")
            .blacklist_function("RtlInitializeSListHead")
            .blacklist_function("RtlInterlockedFlushSList")
            .blacklist_function("RtlInterlockedPopEntrySList")
            .blacklist_function("RtlInterlockedPushEntrySList")
            .blacklist_function("RtlInterlockedPushListSListEx")
            .blacklist_function("RtlQueryDepthSList")
            .blacklist_function("RtlRestoreContext")
            .blacklist_function("RtlUnwindEx")
            .blacklist_function("RtlVirtualUnwind")
            .blacklist_function("SetThreadContext")
            .blacklist_function("SetUnhandledExceptionFilter")
            .blacklist_function("SetXStateFeaturesMask")
            .blacklist_function("UnhandledExceptionFilter")
            .blacklist_function("__C_specific_handler")
            .clang_arg("-DPD_INTERNAL")
            .clang_arg("-DMSW")
            .clang_arg("-DNT")
            .clang_arg("-DWIN32")
            .clang_arg("-DWINDOWS")
            .clang_arg("-D_WINDOWS")
    };

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindgen
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}
