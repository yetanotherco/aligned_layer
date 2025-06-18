use std::{env, path::PathBuf, process::Command};

const GO_SRC: &str = "./gnark/verifier.go";
const GO_OUT: &str = "libverifier.a";
const GO_LIB: &str = "verifier";

const CIRCOM_SRC: &str = "./circom/verifier.go";
const CIRCOM_OUT: &str = "libcircomverifier.a";
const CIRCOM_LIB: &str = "circomverifier";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Fix the missing dependency issue
    let mut get_cmd = Command::new("go");
    get_cmd.arg("get")
           .arg("github.com/yetanotherco/go-circom-prover-verifier/parsers@v0.0.0-20250618180418-d2d74c2e8fd9");
    
    let _ = get_cmd.output(); // Run but don't fail if it has issues
    
    // Build gnark library
    let mut go_build = Command::new("go");
    go_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_dir.join(GO_OUT))
        .arg(GO_SRC);

    go_build.status().expect("Go build failed");

    // Build circom library
    let mut circom_build = Command::new("go");
    circom_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_dir.join(CIRCOM_OUT))
        .arg(CIRCOM_SRC);
    
    let output = circom_build.output().expect("Failed to execute Circom build command");
    if !output.status.success() {
        eprintln!("Circom build failed: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Circom build failed");
    }

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
