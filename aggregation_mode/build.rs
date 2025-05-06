// Reference: https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#advanced-build-options-1
fn main() {
    sp1_build::build_program_with_args("./aggregation_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./aggregation_programs/sp1/elf".to_string()),
            // We use Docker to generate a reproducible ELF that will be identical across all platforms
            // (https://docs.succinct.xyz/docs/sp1/writing-programs/compiling#production-builds)
            docker: true,
            ..Default::default()
        }
    });

    risc0_build::embed_methods();
}
