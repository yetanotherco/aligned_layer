fn main() {
    sp1_build::build_program_with_args("./aggregation_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./aggregation_programs/sp1/elf".to_string()),
            binaries: vec![
                "sp1_chunk_aggregator_program".into(),
                "sp1_root_aggregator_program".into(),
            ],
            ..Default::default()
        }
    });

    risc0_build::embed_methods();
}
