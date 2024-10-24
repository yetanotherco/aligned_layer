use sp1_sdk::{utils, ProverClient, SP1Stdin};
use std::io::Write;

/// The ELF we want to execute inside the zkVM.
const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup logging.
    utils::setup_logger();

    // Create an input stream and write '500' to it.
    let n = 500u32;

    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Generate the proof for the given program and input.
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let mut proof = client.prove(&pk, stdin).compressed().run().unwrap();

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
    let proof_file_path = "../../sp1_fibonacci_new.proof";
    proof.save(proof_file_path).expect("saving proof failed");
    let elf_file_path = "../../sp1_fibonacci_new.elf";
    let mut file = std::fs::File::create(elf_file_path).unwrap();
    file.write_all(ELF).unwrap();

    println!("Successfully generated and verified proof for the program!")
}
