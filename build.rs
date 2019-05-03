//! Download and build Detours, as needed.
use std::{
    env,
    fs::{self, OpenOptions},
    io::{prelude::*, BufWriter},
    path::{Path, PathBuf},
    process::Command,
};

// fn get_nmake() -> PathBuf {
//     let vswhere = Path::new(&env::var_os("ProgramFiles(x86)").unwrap())
//         .join(r"Microsoft Visual Studio\Installer\vswhere.exe");
//     let cmd = Command::new(vswhere)
//         .arg("-latest")
//         .arg("-utf8")
//         .arg("-format")
//         .arg("json")
//         .arg("-find")
//         .arg(r"VC\Tools\**\x64\nmake.exe")
//         .output()
//         .expect("Failed to run vswhere");
//     let nmake = cmd.stdout;
//     // Parse the string of data into serde_json::Value.
//     let nmake: serde_json::Value = serde_json::from_slice(&nmake).unwrap();
//     let nmake = nmake[0].as_str().expect("Couldn't find nmake");
//     PathBuf::from(nmake)
// }

// #[allow(dead_code)]
// fn build_detours() {
//     let nmake = get_nmake();
//     //
//     let cmd = Command::new(nmake)
//         .envs(
//             cc::Build::new()
//                 .cargo_metadata(false)
//                 .get_compiler()
//                 .env()
//                 .to_vec(),
//         )
//         .current_dir(Path::new("deps/detours-src/src"))
//         .arg("/NOLOGO")
//         .output()
//         .expect("Failed to build detours");
//     //
//     let out = cmd.stdout;
//     println!("Final Output:");
//     std::io::stdout().write_all(&out).unwrap();
// }

fn main() {
    // build_detours();
    // unimplemented!();
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
    println!("cargo:rerun-if-changed=build.rs");
    println!("{:#?}", env::var_os("OUT_DIR"));
    //
    unimplemented!()
}
