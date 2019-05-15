//! Build detours.
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=detours");
    //
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("deps/detours-src/src/detours.h", out_path.join("detours.h")).unwrap();
    //
    cc::Build::new()
        .include("deps/detours-src/src/")
        .static_crt(true)
        .flag("/MT")
        .flag("/W4")
        .flag("/WX")
        .flag("/Gy")
        .flag("/Gm-")
        .flag("/Zl")
        .flag("/Od")
        .define("WIN32_LEAN_AND_MEAN", "1")
        .define("_WIN32_WINNT", "0x501")
        .file("deps/detours-src/src/detours.cpp")
        .file("deps/detours-src/src/modules.cpp")
        .file("deps/detours-src/src/disasm.cpp")
        .file("deps/detours-src/src/image.cpp")
        .file("deps/detours-src/src/creatwth.cpp")
        .compile("detours");

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", out_path.to_str().unwrap()))
        .clang_arg("-fms-compatibility")
        .clang_arg("-fms-extensions")
        //
        .whitelist_function("DetourTransactionBegin")
        .whitelist_function("DetourUpdateThread")
        .whitelist_function("DetourAttach")
        .whitelist_function("DetourAttachEx")
        .whitelist_function("DetourDetach")
        .whitelist_function("DetourSetIgnoreTooSmall")
        .whitelist_function("DetourSetRetainRegions")
        .whitelist_function("DetourSetSystemRegionLowerBound")
        .whitelist_function("DetourSetSystemRegionUpperBound")
        .whitelist_function("DetourTransactionAbort")
        .whitelist_function("DetourTransactionCommit")
        .whitelist_function("DetourTransactionCommitEx")
        //
        .whitelist_function("DetourCreateProcessWithDllEx")
        .whitelist_function("DetourCreateProcessWithDlls")
        .whitelist_function("DetourCopyPayloadToProcess")
        .whitelist_function("DetourFinishHelperProcess")
        .whitelist_function("DetourIsHelperProcess")
        .whitelist_function("DetourRestoreAfterWith")
        //
        .header("build/wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
