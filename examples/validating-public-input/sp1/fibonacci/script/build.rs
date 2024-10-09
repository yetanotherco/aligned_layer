use sp1_build::{build_program_with_args, BuildArgs};

fn main() {
    let args = BuildArgs {
        docker: true,
        output_directory: "../".to_string(),
        elf_name: "./fibonacci/sp1_fibonacci.elf".to_string(),
        ..Default::default()
    };
    build_program_with_args("../program", args);
}
