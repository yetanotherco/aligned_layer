use sp1_sdk::{include_elf, utils, HashableKey, ProverClient, SP1Stdin};
use std::io::Write;

/// The ELF we want to execute inside the zkVM.
// const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");
const ELF: &[u8] = include_elf!("fibonacci-program");

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

    // Read and verify the output.
    let _ = proof.public_values.read::<u32>();
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();

    println!("a: {}", a);
    println!("b: {}", b);

    // Verify proof and public values
    client.verify(&proof, &vk).expect("verification failed");

    // Save the proof.
    let proof_file_path = "../../sp1_fibonacci_4_1_3.proof";
    proof.save(proof_file_path).expect("saving proof failed");

    std::fs::write("../../sp1_fibonacci_4_1_3.pub", proof.public_values)
        .expect("failed to save public inputs");

    std::fs::write("../../sp1_fibonacci_4_1_3.vk", vk.hash_bytes())
        .expect("failed to save vk hash");

    let elf_file_path = "../../sp1_fibonacci_4_1_3.elf";
    let mut file = std::fs::File::create(elf_file_path).unwrap();
    file.write_all(ELF).unwrap();

    println!("Successfully generated and verified proof for the program!")
}
