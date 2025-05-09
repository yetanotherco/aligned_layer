fn main() {
    sp1_build::build_program_with_args("./zkvm_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./zkvm_programs/sp1/elf".to_string()),
            ..Default::default()
        }
    });
}
