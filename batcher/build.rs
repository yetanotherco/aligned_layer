use std::{path::PathBuf, env, process::Command};

const GO_SRC: &str = "./gnark/verifier.go";
const GO_OUT: &str = "libgo.a";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut go_build = Command::new("go");
    go_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_dir.join(GO_OUT))
        .arg(GO_SRC);

    go_build.status().expect("Go build failed");

    println!("cargo:rerun-if-changed={}", GO_SRC);
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=go");
}