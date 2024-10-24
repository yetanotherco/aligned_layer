use sp1_build::{build_program_with_args, BuildArgs};

fn main() {
    // This ensures that is the elf file gets removed or changed, the build script will be re-run.
    println!("cargo::rerun-if-changed=../sp1_fibonacci.elf");

    let args = BuildArgs {
        docker: true,
        output_directory: "../".to_string(),
        elf_name: "./fibonacci/sp1_fibonacci.elf".to_string(),
        ..Default::default()
    };
    build_program_with_args("../program", args);
}
