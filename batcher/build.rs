use std::{env, path::PathBuf, process::Command};

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
    // add link flags if linux
    // -ldl -lrt -lm -lssl -lcrypto -Wl,--allow-multiple-definition

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=dl");
        println!("cargo:rustc-link-lib=dylib=rt");
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=ssl");
        println!("cargo:rustc-link-lib=dylib=crypto");
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    }

    println!("cargo:rustc-link-lib=static=go");
}
