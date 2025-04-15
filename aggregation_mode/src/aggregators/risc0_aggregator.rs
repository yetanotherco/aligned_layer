include!(concat!(env!("OUT_DIR"), "/methods.rs"));

use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, Receipt};

use super::lib::{AggregatedProof, ProgramOutput, ProofAggregationError};

pub struct Risc0ProofWithPubValuesAndImageId {
    pub image_id: [u32; 8],
    pub receipt: Receipt,
    pub public_values: Vec<u8>,
}

impl Risc0ProofWithPubValuesAndImageId {
    pub fn hash_image_id_and_public_inputs(&self) -> [u8; 32] {
        [0u8; 32]
    }
}

pub struct Risc0AggregationInput {
    pub receipts: Vec<Risc0ProofWithPubValuesAndImageId>,
    pub merkle_root: [u8; 32],
}

pub(crate) fn aggregate_proofs(
    input: Risc0AggregationInput,
) -> Result<ProgramOutput, ProofAggregationError> {
    let mut env_builder = ExecutorEnv::builder();

    // write assumptions and proof image id + pub inputs
    let mut proofs_image_id_and_pub_inputs = vec![];
    for r in input.receipts {
        proofs_image_id_and_pub_inputs.push(risc0_aggregation_program::Risc0ImageIdAndPubInputs {
            image_id: r.image_id,
            public_inputs: r.public_values,
        });
        env_builder.add_assumption(r.receipt);
    }

    // write input data
    let input = risc0_aggregation_program::Input {
        merkle_root: input.merkle_root,
        proofs_image_id_and_pub_inputs,
    };
    env_builder.write(&input).unwrap();

    let env = env_builder.build().unwrap();

    let prover = default_prover();
    let receipt = prover
        .prove_with_opts(env, RISC0_AGGREGATOR_PROGRAM_ELF, &ProverOpts::groth16())
        .unwrap()
        .receipt;

    Ok(ProgramOutput::new(AggregatedProof::Risc0(receipt)))
}

#[derive(Debug)]
pub enum AlignedRisc0VerificationError {
    Verification,
    UnsupportedProof,
}

pub(crate) fn verify(receipt: &Receipt) -> Result<(), AlignedRisc0VerificationError> {
    // TODO validate and verify receipt is of type Compressed, as only they can be aggregated recursively
    Ok(())
}
