use std::{env, path::PathBuf, process::Command};

const GO_SRC: &str = "./gnark/verifier.go";
const GO_OUT: &str = "libverifier.a";
const GO_LIB: &str = "verifier";

const NEXUS_DIR: &str = "../../operator/nexus/lib";
const NEXUS_OUT: &str = "libnexus_verifier.a";
const NEXUS_LIB: &str = "nexus_verifier";

fn main() {
    println!("cargo:rerun-if-changed={}", NEXUS_DIR);
    println!("cargo:rerun-if-changed={}", GO_SRC);

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    }
    println!("cargo:rustc-link-arg=-Wl,-hidden-l{},-exported_symbol,_verify_nexus_proof_ffi", NEXUS_LIB);

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.to_str().unwrap()
    );
    let nexus_out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("nexus");
    println!(
        "cargo:rustc-link-search=native={}",
        nexus_out_dir.to_str().unwrap()
    );

    let mut go_build = Command::new("go");
    go_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_dir.join(GO_OUT))
        .arg(GO_SRC);

    go_build.status().expect("Go build failed");

    let mut ffi_build = Command::new("make");
    ffi_build
        .arg("-C")
        .arg("..")
        .arg("-j")
        .arg("build_all_ffi");

    ffi_build.status().expect("FFI build failed");

    println!("cargo:rustc-link-lib=static={}", GO_LIB);
    //println!("cargo:rustc-link-lib=static={}", NEXUS_LIB);
}
