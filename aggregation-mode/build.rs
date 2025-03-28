fn main() {
    sp1_build::build_program_with_args("./zkvm/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./zkvm/sp1/elf".to_string()),
            ..Default::default()
        }
    })
}
