use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_bytes!("../../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let n = 1000u32;
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Setup the program for proving.
    let (pk, vk) = client.setup(FIBONACCI_ELF);

    // // Generate the proof
    let mut proof = client
        .prove(&pk, stdin)
        .compressed()
        .run()
        .expect("failed to generate proof");
    println!("Successfully generated proof!");

    let (a, b): (u32, u32) = proof.public_values.read();

    println!("a: {}", a);
    println!("b: {}", b);

    // Verify the proof.
    client.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!");

    proof
        .save("../sp1_fibonacci.proof")
        .expect("failed to save proof");
    std::fs::write("../sp1_fibonacci.pub", proof.public_values)
        .expect("failed to save public input");
    std::fs::write("../sp1_fibonacci.elf", FIBONACCI_ELF)
        .expect("failed to save elf file");
}
