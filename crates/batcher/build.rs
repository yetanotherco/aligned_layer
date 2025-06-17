use std::{env, path::PathBuf, process::Command};

const GO_SRC: &str = "./gnark/verifier.go";
const GO_OUT: &str = "libverifier.a";
const GO_LIB: &str = "verifier";

const CIRCOM_SRC: &str = "./circom/verifier.go";
const CIRCOM_OUT: &str = "libcircomverifier.a";
const CIRCOM_LIB: &str = "circomverifier";

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

    let mut circom_build = Command::new("go");
    circom_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_dir.join(CIRCOM_OUT))
        .arg(CIRCOM_SRC);

    circom_build.status().expect("Circom build failed");

    println!("cargo:rerun-if-changed={}", GO_SRC);
    println!("cargo:rerun-if-changed={}", CIRCOM_SRC);
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    }

    println!("cargo:rustc-link-lib=static={}", GO_LIB);
    println!("cargo:rustc-link-lib=static={}", CIRCOM_LIB);
}
