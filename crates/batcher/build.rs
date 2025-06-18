
use std::{env, path::PathBuf, process::Command};

const GO_SRC: &str = "./gnark/verifier.go";
const GO_OUT: &str = "libverifier.a";
const GO_LIB: &str = "verifier";

const CIRCOM_SRC: &str = "./circom/verifier.go";
const CIRCOM_OUT: &str = "libcircomverifier.a";
const CIRCOM_LIB: &str = "circomverifier";

fn ensure_go_dependencies() {
    // Check if there's a go.mod in the current directory or parent directories
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Build script running in: {:?}", current_dir);
    
    // Try to find go.mod files
    let possible_dirs = vec![
        current_dir.clone(),
        current_dir.parent().unwrap_or(&current_dir).to_path_buf(),
        current_dir.parent().unwrap_or(&current_dir).parent().unwrap_or(&current_dir).to_path_buf(),
    ];
    
    for dir in possible_dirs {
        let go_mod_path = dir.join("go.mod");
        if go_mod_path.exists() {
            println!("Found go.mod in: {:?}", dir);
            
            // Run go mod tidy to ensure all dependencies are properly resolved
//             let mut tidy_cmd = Command::new("go");
//             tidy_cmd.arg("mod").arg("tidy").current_dir(&dir);
            
//             let tidy_output = tidy_cmd.output().expect("Failed to run go mod tidy");
//             if !tidy_output.status.success() {
//                 eprintln!("go mod tidy failed in {:?}: {}", dir, String::from_utf8_lossy(&tidy_output.stderr));
//             } else {
//                 println!("Successfully ran go mod tidy in {:?}", dir);
//             }
            
            // Run go mod download to ensure all modules are cached
            let mut cmd = Command::new("go");
            cmd.arg("mod").arg("download").current_dir(&dir);
            
            let output = cmd.output().expect("Failed to run go mod download");
            if !output.status.success() {
                eprintln!("go mod download failed in {:?}: {}", dir, String::from_utf8_lossy(&output.stderr));
            } else {
                println!("Successfully ran go mod download in {:?}", dir);
            }
            
            break;
        }
    }
}

fn build_go_library(src: &str, out: &str, out_dir: &PathBuf) {
    println!("Building Go library: {} -> {}", src, out);
    
    let mut go_build = Command::new("go");
    go_build
        .arg("build")
        .arg("-buildmode=c-archive")
        .arg("-o")
        .arg(out_dir.join(out))
        .arg(src);

    let output = go_build.output().expect("Failed to execute Go build command");
    if !output.status.success() {
        eprintln!("Go build failed for {}: {}", src, String::from_utf8_lossy(&output.stderr));
        panic!("Go build failed for {}", src);
    }
    println!("Successfully built {}", src);
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Ensure Go dependencies are available
    ensure_go_dependencies();
    
    // Build both Go libraries
    build_go_library(GO_SRC, GO_OUT, &out_dir);
    build_go_library(CIRCOM_SRC, CIRCOM_OUT, &out_dir);

    println!("cargo:rerun-if-changed={}", GO_SRC);
    println!("cargo:rerun-if-changed={}", CIRCOM_SRC);
    println!("cargo:rerun-if-changed=go.mod");
    println!("cargo:rerun-if-changed=go.sum");
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
