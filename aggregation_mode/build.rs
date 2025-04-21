fn main() {
    sp1_build::build_program_with_args("./aggregation_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./aggregation_programs/sp1/elf".to_string()),
            ..Default::default()
        }
    });

    risc0_build::embed_methods();
}
