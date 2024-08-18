fn main() {
    println!("cargo:rerun-if-changed=src/cool.h");
    println!("cargo:rerun-if-changed=src/cool.c");

    bindgen::Builder::default()
        .header("src/cool.h")
        .wrap_unsafe_ops(true)
        .generate()
        .unwrap()
        .write_to_file("src/cool.rs")
        .unwrap();

    cc::Build::new()
        .file("src/cool.c")
        .compile("libcool");
}
