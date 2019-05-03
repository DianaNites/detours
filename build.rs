//! Download and build Detours, as needed.
use std::{
    env,
    fs::{self, OpenOptions},
    io::{prelude::*, BufWriter},
    path::{Path, PathBuf},
};
use std::io;
use flate2::read::DeflateDecoder;

static DOWNLOAD_URL: &str = "https://github.com/Microsoft/Detours/archive/v4.0.1.zip";

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

/// Download sources, return path to zip.
fn download_sources() -> Result<PathBuf> {
    //
    let path = Path::new(&env::var_os("OUT_DIR").unwrap()).join("Detours.zip");
    println!("{:#?}", path);
    //
    let mut resp = reqwest::get(DOWNLOAD_URL)?;
    let mut buf = Vec::new();
    resp.read_to_end(&mut buf);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path).expect("Couldn't create Detours.zip");
    let mut writer = BufWriter::new(file);
    writer.write_all(&mut buf);
    Ok(path)
}

fn main() {
    let src = download_sources().expect("Couldn't get Detours source.");
}
