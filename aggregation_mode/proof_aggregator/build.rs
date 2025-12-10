use risc0_build::{DockerOptionsBuilder, GuestOptionsBuilder};
use std::collections::HashMap;
use std::path::PathBuf;

// This allows us to skip the guest build in CI or local environments where it's not needed (reducing the build time)
// Note: To use this flag, the aggregation programs should be already compiled, otherwise the compilation will be done anyway.
fn should_skip_build() -> bool {
    if std::env::var("SKIP_AGG_PROGRAMS_BUILD")
        .map(|v| v == "1")
        .unwrap_or(false)
    {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let methods_path = PathBuf::from(out_dir).join("methods.rs");

        methods_path.exists()
    } else {
        false
    }
}

// Reference: https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#advanced-build-options-1
fn main() {
    if should_skip_build() {
        println!("cargo:warning=SKIP_AGG_PROGRAMS_BUILD=1: methods.rs already exists, skipping aggregation programs build");
        return;
    } else {
        println!("cargo:warning=SKIP_AGG_PROGRAMS_BUILD=1 set, but path does not exist, running full build");
    }

    sp1_build::build_program_with_args("./aggregation_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./aggregation_programs/sp1/elf".to_string()),
            binaries: vec![
                "sp1_user_proofs_aggregator_program".into(),
                "sp1_chunk_aggregator_program".into(),
            ],
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
}
