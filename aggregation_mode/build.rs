use risc0_build::{DockerOptionsBuilder, GuestOptionsBuilder};
use sha3::{Digest, Keccak256};

use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::{env, fs};

fn hash_files_and_features<P: AsRef<Path>>(paths: &[P], features: Vec<String>) -> String {
    let mut hasher = Keccak256::new();
    for path in paths {
        let mut f = fs::File::open(path).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        hasher.update(&buffer);
    }

    for feature in features {
        hasher.update(&feature);
    }

    format!("{:x}", hasher.finalize())
}

// Reference: https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#advanced-build-options-1
fn main() {
    let programs = [
        "build.rs",
        "aggregation_programs/Cargo.toml",
        "aggregation_programs/Cargo.lock",
        "aggregation_programs/sp1/Cargo.toml",
        "aggregation_programs/sp1/src/lib.rs",
        "aggregation_programs/sp1/src/user_proofs_aggregator_main.rs",
        "aggregation_programs/sp1/src/chunk_aggregator_main.rs",
        "aggregation_programs/risc0/Cargo.toml",
        "aggregation_programs/risc0/src/user_proofs_aggregator_main.rs",
        "aggregation_programs/risc0/src/chunk_aggregator_main.rs",
        "aggregation_programs/risc0/src/lib.rs",
    ];

    for file in &programs {
        println!("cargo:rerun-if-changed={}", file);
    }

    // Get all the env vars from rust (RUSTC, CARGO_FEATURES, etc) 
    // But filter those that don't affect the build of the program
    let mut flags: Vec<String> = env::vars()
        .filter(|(k, _)| k != "AGGREGATOR" || k != "RISC0_DEV_MODE" || k != "SP1_PROVER")
        .map(|(k, v)| format!("{k}={v}"))
        .collect();
    // Sort them to make it deterministic in spite of the order.
    flags.sort();

    let hash = hash_files_and_features(&programs, flags);
    let hash_file = Path::new("target/programs_hash.txt");

    let needs_build = if let Ok(prev) = fs::read_to_string(hash_file) {
        prev != hash
    } else {
        true
    };

    if needs_build {
        sp1_build::build_program_with_args("./aggregation_programs/sp1", {
            sp1_build::BuildArgs {
                output_directory: Some("./aggregation_programs/sp1/elf".to_string()),
                // We use Docker to generate a reproducible ELF that will be identical across all platforms
                // (https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#production-builds)
                docker: true,
                ..Default::default()
            }
        });

        // With this containerized build process, we ensure that all builds of the guest code,
        // regardless of the machine or local environment, will produce the same ImageID
        let docker_options = DockerOptionsBuilder::default().build().unwrap();
        // Reference: https://github.com/risc0/risc0/blob/main/risc0/build/src/config.rs#L73-L90
        let guest_options = GuestOptionsBuilder::default()
            .use_docker(docker_options)
            .build()
            .unwrap();
        risc0_build::embed_methods_with_options(HashMap::from([(
            "risc0_aggregation_program",
            guest_options,
        )]));

        fs::write(hash_file, hash).unwrap();
    } else {
        println!("Programs code unchanged — skipping programs build");
    }
}
