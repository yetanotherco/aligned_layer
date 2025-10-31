use sp1_sdk::{HashableKey, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_bytes!("../../sp1_fibonacci.elf");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let n = 500;
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Setup the program for proving.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // // Generate the proof
    let proof = client
        .prove(&pk, &stdin)
        .compressed()
        .run()
        .expect("failed to generate proof");
    println!("Successfully generated proof!");

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");

    // Print ELF
    println!("{}", hex::encode(vk.hash_bytes()));

    proof
        .save("../sp1_fibonacci.proof")
        .expect("failed to save proof");
    std::fs::write("../sp1_fibonacci.pub", proof.public_values)
        .expect("failed to save public input");
}
