use risc0_build::{DockerOptionsBuilder, GuestOptionsBuilder};
use std::collections::HashMap;

// Reference: https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#advanced-build-options-1
fn main() {
    // With this containerized build process, we ensure that all builds of the guest code,
    // regardless of the machine or local environment, will produce the same ImageID
    let docker_options = DockerOptionsBuilder::default().build().unwrap();
    // Reference: https://github.com/risc0/risc0/blob/main/risc0/build/src/config.rs#L73-L90
    let guest_options = GuestOptionsBuilder::default()
        .use_docker(docker_options)
        .build()
        .unwrap();

    risc0_build::embed_methods_with_options(HashMap::from([(
        "fibonacci",
        guest_options,
    )]));
}