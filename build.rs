use std::{env, path};

fn main() {
    let out_path = path::PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!(
        "cargo:rustc-link-lib={}",
        env::var("ASTCENC_LIB_PATH").unwrap()
    );
    let include_path = env::var("ASTCENC_HEADER_PATH").unwrap();

    // Link to libstdc++ on GNU
    let target = env::var("TARGET").unwrap();
    if target.contains("gnu") {
        println!("cargo:rustc-link-lib=stdc++");
    } else if target.contains("apple") {
        println!("cargo:rustc-link-lib=c++");
    }

    let bindings = bindgen::Builder::default()
        .clang_arg("-xc++")
        .header(&include_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_debug(true)
        // Bypasses an issue with bindgen that makes it generate invalid Rust code.
        .blocklist_item("std::value");

    let bindings = bindings.generate().expect("Unable to generate bindings");

    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings");

    println!("cargo:rerun-if-changed=build.rs");
}
