// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{FIBONACCI_ELF, FIBONACCI_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

const PROOF_FILE_PATH: &str = "risc_zero_fibonacci_old.proof";
const PUB_INPUT_FILE_PATH: &str = "risc_zero_fibonacci_old.pub";
const FIBONACCI_ID_FILE_PATH: &str = "fibonacci_id_old.bin";

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    // For example:
    let input: u32 = 500;
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, FIBONACCI_ELF).unwrap().receipt;

    // Retrieve receipt journal here.
    let vars: (u32, u32) = receipt.journal.decode().unwrap();

    let (a, b) = vars;

    println!("a: {}", a);
    println!("b: {}", b);

    let verification_result = receipt.verify(FIBONACCI_ID).is_ok();

    println!("Verification result: {}", verification_result);

    let serialized = bincode::serialize(&receipt.inner).expect("Failed to serialize the receipt");

    std::fs::write(PROOF_FILE_PATH, serialized).expect("Failed to write proof file");

    std::fs::write(FIBONACCI_ID_FILE_PATH, convert(&FIBONACCI_ID))
        .expect("Failed to write fibonacci_id file");

    std::fs::write(PUB_INPUT_FILE_PATH, receipt.journal.bytes)
        .expect("Failed to write pub_input file");
}

pub fn convert(data: &[u32; 8]) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..8 {
        res[4 * i..4 * (i + 1)].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}
