// use cc;
// extern crate cc;
#[cfg(feature = "bindgen")]
extern crate bindgen;
use std::path::PathBuf;
use std::{ fs, env };

fn main() {
    let resonance_libs_dir = env::var("resonance_LIBS")
        .unwrap_or_else(|_| {
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("lib")
            .to_string_lossy().to_string()
        });
    println!("cargo:rustc-link-lib=dylib=ResonanceAudioStatic");
    /*
    let mut libs = vec!["inspector", "v8_base_0", "v8_base_1", "v8_external_snapshot", "v8_init", "v8_initializers", "v8_libbase", "v8_libplatform", "v8_libsampler", "v8_nosnapshot", "v8_snapshot"];
    if cfg!(windows) {
        libs.push("dbghelp");
        libs.push("shlwapi");
        libs.push("winmm");
    }
    // let libs = ["v8.dll", "v8_libbase.dll", "v8_libplatform.dll"];
    for l in &libs {
        println!("cargo:rustc-link-lib=dylib={}", l);
    }
    */
    println!("cargo:rustc-link-search={}", resonance_libs_dir);
/*    
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .include(v8_includes_dir)
        .file("src/allocator.cpp")
        .file("src/isolate.cpp")
        .file("src/platform.cpp")
        .compile("librust-v8-impls.a");
    
    // println!("cargo:rustc-link-lib=dylib=librust-v8-impls.a");
  */  
    #[cfg(feature = "bindgen")] {
        generate_bindings();
    }
    
    #[cfg(not(feature = "bindgen"))] {
        copy_pregenerated_bindings();
    }
}

#[cfg(feature = "bindgen")]
fn generate_bindings() {
    use std::path;
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let crate_path_str = crate_path.to_str().unwrap();
    let mut resonance_include_dir = crate_path.clone();
    resonance_include_dir.push("src");
    resonance_include_dir.push("resonance");
    let mut third_party_include_dir = crate_path.clone();
    third_party_include_dir.push("src");
    third_party_include_dir.push("third_party");
    let bindings = bindgen::Builder::default()
        .generate_comments(true)
        .header("src/wrapper.hpp")
        .rust_target(bindgen::RustTarget::Nightly)
        .clang_arg("--std=c++14")
        .clang_arg(format!("--include-directory={}/src/resonance", crate_path_str))
        .clang_arg(format!("--include-directory={}/src/third_party/eigen", crate_path_str))
        .clang_arg(format!("--include-directory={}/src/third_party/pffft", crate_path_str))
        // Because there are some layout problems with these
        .opaque_type("std::.*")
        // .blacklist_type("Eigen::.*")
        // .whitelist_type("Eigen::AngleAxis")
        // .blacklist_type("PFFFT.+")
        // .whitelist_type("PFFFT::PFFFT_Setup")
        .whitelist_type("vraudio::.*")
        .whitelist_type("rust_v8_impls::.*")
        .whitelist_function("vraudio::.*")
        .whitelist_function("rust_v8_impls::.*")
        .whitelist_var("vraudio::.*")
        .whitelist_var("rust_v8_impls::.*")
        // Re-structure the modules a bit and hide the "root" module
        // .raw_line("#[doc(hidden)]")
        // .generate_inline_functions(true)
        .enable_cxx_namespaces()
        .derive_debug(true)
        .derive_hash(true)
        .derive_eq(true)
        .derive_partialeq(true)
        .rustfmt_bindings(true)
        .generate()
        .expect("unable to generate resonance audio bindings");

    let out_path = path::PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR env var not set"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("unable to write bindings file");
    
    fs::copy(out_path.join("bindings.rs"), crate_path.join("pregenerated_bindings.rs"))
        .expect("Couldn't find generated bindings!");
}

#[cfg(not(feature = "bindgen"))]
fn copy_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    fs::copy(crate_path.join("pregenerated_bindings.rs"), out_path.join("bindings.rs"))
        .expect("Couldn't find pregenerated bindings!");
}