use crate::ffi::gnark_ffi::{
    VerifyGroth16ProofBN254, VerifyPlonkProofBLS12_381, VerifyPlonkProofBN254,
};
use aligned_sdk::common::types::ProvingSystemId;

pub fn verify_gnark(
    proving_system: &ProvingSystemId,
    proof: &Vec<u8>,
    public_input: &Vec<u8>,
    verification_key: &Vec<u8>,
) -> bool {
    let proof = proof.into();
    let public_input = public_input.into();
    let verification_key = verification_key.into();

    match proving_system {
        ProvingSystemId::GnarkPlonkBn254 => unsafe {
            VerifyPlonkProofBN254(proof, public_input, verification_key)
        },
        ProvingSystemId::GnarkPlonkBls12_381 => unsafe {
            VerifyPlonkProofBLS12_381(proof, public_input, verification_key)
        },
        ProvingSystemId::Groth16Bn254 => unsafe {
            VerifyGroth16ProofBN254(proof, public_input, verification_key)
        },
        _ => false,
    }
}
