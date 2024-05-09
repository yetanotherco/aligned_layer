use sp1_sdk::{utils, Groth16Proof, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

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
    /*
    let mut proof = client.prove_groth16(&pk, stdin).unwrap();

    println!("generated proof");

    // Read and verify the output.
    let _ = proof.public_values.read::<u32>();
    let a = proof.public_values.read::<u32>();
    let b = proof.public_values.read::<u32>();
    println!("a: {}", a);
    println!("b: {}", b);

    */

    let proof = SP1ProofWithPublicValues::<Groth16Proof>::load("../../fibonacci_data/proof-with-pis-edited.json").unwrap();
    client
        .verify_groth16(&proof, &vk)
        .expect("verification failed");

    // Save the proof.
    /*
    proof
        .save("proof-with-pis.json")
        .expect("saving proof failed");
    */
    println!("successfully verified proof for program!")
}