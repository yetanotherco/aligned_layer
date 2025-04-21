// Paths where the generated Solidity files will be written.
const SOLIDITY_IMAGE_ID_PATH: &str = "../contracts/src/ImageID.sol";
const SOLIDITY_ELF_PATH: &str = "../contracts/src/Elf.sol";

fn main() {
    sp1_build::build_program_with_args("./aggregation_programs/sp1", {
        sp1_build::BuildArgs {
            output_directory: Some("./aggregation_programs/sp1/elf".to_string()),
            ..Default::default()
        }
    });

    let guests = risc0_build::embed_methods();

    let solidity_opts = risc0_build_ethereum::Options::default()
        .with_image_id_sol_path(SOLIDITY_IMAGE_ID_PATH)
        .with_elf_sol_path(SOLIDITY_ELF_PATH);

    risc0_build_ethereum::generate_solidity_files(guests.as_slice(), &solidity_opts).unwrap();
}
