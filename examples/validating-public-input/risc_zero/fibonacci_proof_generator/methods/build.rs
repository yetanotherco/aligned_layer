use std::{collections::HashMap, env, path::PathBuf};

use risc0_build::{DockerOptions, GuestOptions};

fn main() {
    match env::current_dir() {
        Ok(current_dir) => {
            if let Some(parent) = current_dir.parent() {
                let parent_path = PathBuf::from(parent);
                // Set the root directory for Docker to risc_zero/fibonacci_proof_generator

                let docker_options = Some(DockerOptions {
                    root_dir: Some(parent_path),
                });

                let guest_options = HashMap::from([(
                    "fibonacci",
                    GuestOptions {
                        features: vec![],
                        use_docker: docker_options,
                    },
                )]);

                risc0_build::embed_methods_with_options(guest_options);
            } else {
                println!("The current directory does not have a parent.");
            }
        }
        Err(e) => println!("Error getting current directory: {}", e),
    }
}
