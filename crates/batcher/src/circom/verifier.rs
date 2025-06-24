use crate::ffi::circom_ffi::VerifyCircomGroth16ProofBN128;
use aligned_sdk::common::types::ProvingSystemId;

pub fn verify_circom(
    proving_system: &ProvingSystemId,
    proof: &Vec<u8>,
    public_input: &Vec<u8>,
    verification_key: &Vec<u8>,
) -> bool {
    let proof = proof.into();
    let public_input = public_input.into();
    let verification_key = verification_key.into();

    match proving_system {
        ProvingSystemId::CircomGroth16Bn128 => unsafe {
            VerifyCircomGroth16ProofBN128(proof, public_input, verification_key)
        },
        _ => false,
    }
}
