use sp1_build::{build_program_with_args, BuildArgs};

fn main() {
    let args = BuildArgs {
        docker: true,
        ..Default::default()
    };
    build_program_with_args("../program", args);
}
