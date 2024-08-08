use std::collections::HashMap;

use risc0_build::{DockerOptions, GuestOptions};

fn main() {
    let docker_options = Some(DockerOptions { root_dir: None });

    let guest_options = HashMap::from([(
        "risc0-zkvm-methods-guest",
        GuestOptions {
            features: vec![],
            use_docker: docker_options,
        },
    )]);

    risc0_build::embed_methods_with_options(guest_options);
}
