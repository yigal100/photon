use std::io::Write;
use std::{env, io};
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_path = Path::new("src").join("iron.gram");

    let output = Command::new("himecc")
        .args(["--target", "rust", "--embed", "--path", &out_dir])
        .arg(src_path.to_str().unwrap())
        .output().unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    println!("cargo::rerun-if-changed=src/iron.gram");
    println!("cargo::rerun-if-changed=build.rs");
}
