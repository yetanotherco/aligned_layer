use std::{env, path::PathBuf, process::Command};

const GO_SRC: &str = "./gnark/verifier.go";
const GO_OUT: &str = "libverifier.a";
const GO_LIB: &str = "verifier";

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

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    }

    println!("cargo:rustc-link-lib=static={}", GO_LIB);
}
