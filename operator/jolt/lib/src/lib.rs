use jolt_core::{
    jolt::vm::{Jolt, JoltProof, JoltCommitments, JoltPreprocessing, rv32i_vm::RV32IJoltVM},
    poly::commitment::hyrax::HyraxScheme
};
use tracer::decode;
use ark_serialize::CanonicalDeserialize;
use ark_bn254::{Fr, G1Projective};

// MaxProofSize 4MB
pub const MAX_PROOF_SIZE: usize = 4 * 1024 * 1024;

// MaxipaParamsSize 1MB
pub const MAX_COMMITMENT_SIZE: usize = 200 * 1024;

// MaxPublicInputSize 4KB
pub const MAX_ELF_SIZE: usize = 64 * 1024;

#[no_mangle]
pub extern "C" fn verify_jolt_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: u32,
    elf_bytes: &[u8; MAX_ELF_SIZE],
    elf_len: u32,
    commitment_bytes: &[u8; MAX_COMMITMENT_SIZE],
    commitment_len: u32,
) -> bool {
    // Note(pat): To save space we could try using the compressed forms
    if let Ok(jolt_proof) = JoltProof::deserialize_uncompressed(&proof_bytes[.. proof_len as usize]) {
        if let Ok(jolt_commitments) = JoltCommitments::deserialize_uncompressed(&commitment_bytes[..commitment_len as usize]) {
            let (bytecode, memory_init) = decode(&elf_bytes[..elf_len as usize]);

            let preprocessing: JoltPreprocessing<Fr, HyraxScheme<G1Projective>> =
                RV32IJoltVM::preprocess(bytecode.clone(), memory_init, 1 << 20, 1 << 20, 1 << 20);

            let verification_result = RV32IJoltVM::verify(preprocessing, jolt_proof, jolt_commitments);
            return verification_result.is_ok();
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/jolt.proof");
    const ELF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/jolt.elf");
    const COMMITMENT: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/jolt.commitment");

    #[test]
    fn verify_jolt_proof_with_elf_works() {
        let mut proof_buffer = [0u8; MAX_PROOF_SIZE];
        let proof_len = PROOF.len();
        proof_buffer[..proof_len].clone_from_slice(PROOF);

        let mut elf_buffer = [0u8; MAX_ELF_SIZE];
        let elf_len = ELF.len();
        elf_buffer[..elf_len].clone_from_slice(ELF);

        let mut commitment_buffer = [0u8; MAX_COMMITMENT_SIZE];
        let commitment_len = COMMITMENT.len();
        commitment_buffer[..commitment_len].clone_from_slice(COMMITMENT);

        let result = verify_jolt_proof_ffi(&proof_buffer, proof_len as u32, &elf_buffer, elf_len as u32, &commitment_buffer, commitment_len as u32);
        assert!(result)
    }

    #[test]
    fn verify_jolt_aborts_with_bad_proof() {
        let mut proof_buffer = [42u8; super::MAX_PROOF_SIZE];
        let proof_len = PROOF.len();
        proof_buffer[..proof_len].clone_from_slice(PROOF);

        let mut elf_buffer = [0u8; MAX_ELF_SIZE];
        let elf_len = ELF.len();
        elf_buffer[..elf_len].clone_from_slice(ELF);

        let mut commitment_buffer = [0u8; MAX_COMMITMENT_SIZE];
        let commitment_len = COMMITMENT.len();
        commitment_buffer[..commitment_len].clone_from_slice(COMMITMENT);

        let result = verify_jolt_proof_ffi(&proof_buffer, (proof_len - 1) as u32, &elf_buffer, elf_len as u32, &commitment_buffer, commitment_len as u32);
        assert!(!result)
    }
}
