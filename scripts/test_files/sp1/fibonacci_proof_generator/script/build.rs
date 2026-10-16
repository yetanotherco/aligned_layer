use sp1_build::{build_program_with_args, BuildArgs};

fn main() {
    // We use Docker so the program does not depend on the locally installed SP1 toolchain
    build_program_with_args(
        "../program",
        BuildArgs {
            docker: true,
            tag: "v6.9.0".to_string(),
            ..Default::default()
        },
    )
}
