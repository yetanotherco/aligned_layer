fn main() {
    sp1_build::build_program_with_args("./zkvm_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./zkvm_programs/sp1/elf".to_string()),
            // We use Docker so the program does not depend on the locally installed SP1 toolchain
            docker: true,
            tag: "v6.9.0".to_string(),
            // Mount the l2 workspace so the program can reach its `types` path dependency,
            // which inherits its dependencies from the l2 workspace manifest
            workspace_directory: Some("../../".to_string()),
            ..Default::default()
        }
    });
}
