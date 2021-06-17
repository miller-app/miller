fn main() {
    cc::Build::new()
        .include("src/cpp")
        .file("src/cpp/message_obj_wrapper.h")
        .file("src/cpp/message_obj_wrapper.cpp")
        .cpp(true)
        .flag("-std=c++11")
        .compile("miller_wrappers");

    println!("cargo:rerun-if-changed=src/cpp/message_obj_wrapper.h");
    println!("cargo:rerun-if-changed=src/cpp/message_obj_wrapper.cpp");
}
