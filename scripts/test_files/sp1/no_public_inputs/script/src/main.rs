use sp1_sdk::{include_elf, utils, HashableKey, ProverClient, SP1Stdin};
use std::io::Write;

/// The ELF we want to execute inside the zkVM.
// const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
const ELF: &[u8] = include_elf!("no-public-input-program");

const SP1_VERSION: &str = "5_0_0";

fn main() {
    // Setup logging.
    utils::setup_logger();

    // Create an input stream and write '500' to it.
    let n = 500u32;

    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Generate the proof for the given program and input.
    let client = ProverClient::from_env();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, &stdin).compressed().run().unwrap();

    println!("Fibonacci program proof generated");

    // Verify proof and public values
    client.verify(&proof, &vk).expect("verification failed");

    // Save the proof.
    let proof_file_path = format!("../../sp1_no_pub_input_{}.proof", SP1_VERSION);
    proof.save(proof_file_path).expect("saving proof failed");

    std::fs::write(format!("../../sp1_no_pub_input_{}.vk", SP1_VERSION), vk.hash_bytes())
        .expect("failed to save vk hash");

    let elf_file_path = format!("../../sp1_no_pub_input_{}.elf", SP1_VERSION);
    let mut file = std::fs::File::create(elf_file_path).unwrap();
    file.write_all(ELF).unwrap();

    println!("Successfully generated and verified proof for the program!")
}
