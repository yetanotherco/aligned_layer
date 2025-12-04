use risc0_build::{DockerOptionsBuilder, GuestOptionsBuilder};
use std::collections::HashMap;

// Reference: https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#advanced-build-options-1
fn main() {
    // If we are in docker, we don't want to build the programs with docker
    let in_docker = std::env::var("IN_DOCKER").is_ok();

    sp1_build::build_program_with_args("./aggregation_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./aggregation_programs/sp1/elf".to_string()),
            binaries: vec![
                "sp1_user_proofs_aggregator_program".into(),
                "sp1_chunk_aggregator_program".into(),
            ],
            // We use Docker to generate a reproducible ELF that will be identical across all platforms
            // (https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#production-builds)
            docker: !in_docker,
            ..Default::default()
        }
    });

    // With this containerized build process, we ensure that all builds of the guest code,
    // regardless of the machine or local environment, will produce the same ImageID
    let docker_options = DockerOptionsBuilder::default().build().unwrap();
    // Reference: https://github.com/risc0/risc0/blob/main/risc0/build/src/config.rs#L73-L90
    let guest_options = if !in_docker {
        GuestOptionsBuilder::default()
            .use_docker(docker_options)
            .build()
            .unwrap()
    } else {
        GuestOptionsBuilder::default().build().unwrap()
    };

    risc0_build::embed_methods_with_options(HashMap::from([(
        "risc0_aggregation_program",
        guest_options,
    )]));
}
