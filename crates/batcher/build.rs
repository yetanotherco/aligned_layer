use std::{env, path::PathBuf, process::Command};

const GO_SRC: &str = "./go_verifiers_lib/verifier.go";
const GO_OUT: &str = "libverifier.a";
const GO_LIB: &str = "verifier";

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Fix the missing dependency issue
    let mut get_cmd = Command::new("go");
    get_cmd.arg("get")
           .arg("github.com/yetanotherco/go-circom-prover-verifier/parsers@v0.0.0-20250618185957-f01a8a8ec4a6");

    let _ = get_cmd.output(); // Run but don't fail if it has issues

    // Build library
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

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    }

    println!("cargo:rustc-link-lib=static={}", GO_LIB);
}
