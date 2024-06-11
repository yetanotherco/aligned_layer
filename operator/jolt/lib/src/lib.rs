use jolt_core::{
    jolt::vm::{Jolt, JoltPreprocessing, rv32i_vm::RV32IJoltVM},
    poly::commitment::hyrax::HyraxScheme
};
use jolt_sdk::host_utils::Proof;
use tracer::decode;
use ark_serialize::CanonicalDeserialize;
use ark_bn254::{Fr, G1Projective};

// MaxProofSize 4MB
pub const MAX_PROOF_SIZE: usize = 4 * 1024 * 1024;

// MaxPublicInputSize 4KB
pub const MAX_ELF_SIZE: usize = 2 * 1024 * 1024;

#[no_mangle]
pub extern "C" fn verify_jolt_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: u32,
    elf_bytes: &[u8; MAX_ELF_SIZE],
    elf_len: u32,
) -> bool {
    if let Ok(jolt_proof) = Proof::deserialize_compressed(&proof_bytes[.. proof_len as usize]) {
        // Add public inputs...
        //TODO: check if we need to load function and function args... These should be serialized in the VK
        let (bytecode, memory_init) = decode(&elf_bytes[..elf_len as usize]);

        // Note: this VM sizes are based on the hardcoded values in the Jolt codebase: _____
        let preprocessing: JoltPreprocessing<Fr, HyraxScheme<G1Projective>> =
            RV32IJoltVM::preprocess(bytecode, memory_init, 1 << 20, 1 << 20, 1 << 24);

        return RV32IJoltVM::verify(preprocessing, jolt_proof.proof, jolt_proof.commitments).is_ok();
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fibonacci
    const FIB_PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/fibonacci/jolt.proof");
    const FIB_ELF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/fibonacci/jolt.elf");

    // Sha3
    const SHA3_PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/sha3-ex/jolt.proof");
    const SHA3_ELF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/sha3-ex/jolt.elf");

    fn verify_jolt_proof_with_elf_works(proof: &[u8], elf: &[u8]) {
        let mut proof_buffer = [0u8; MAX_PROOF_SIZE];
        let proof_len = proof.len();
        proof_buffer[..proof_len].clone_from_slice(proof);

        let mut elf_buffer = [0u8; MAX_ELF_SIZE];
        let elf_len = elf.len();
        elf_buffer[..elf_len].clone_from_slice(elf);

        let result = verify_jolt_proof_ffi(&proof_buffer, proof_len as u32, &elf_buffer, elf_len as u32);
        assert!(result)
    }

    fn verify_jolt_aborts_with_bad_proof(proof: &[u8], elf: &[u8]) {
        let mut proof_buffer = [42u8; super::MAX_PROOF_SIZE];
        let proof_len = proof.len();
        proof_buffer[..proof_len].clone_from_slice(proof);

        let mut elf_buffer = [0u8; MAX_ELF_SIZE];
        let elf_len = elf.len();
        elf_buffer[..elf_len].clone_from_slice(elf);

        let result = verify_jolt_proof_ffi(&proof_buffer, (proof_len - 1) as u32, &elf_buffer, elf_len as u32);
        assert!(!result)
    }

    #[test]
    fn verify_jolt_fib_e2e_proof_with_elf_works() { 
        verify_jolt_proof_with_elf_works(&FIB_PROOF, &FIB_ELF)
    }

    #[test]
    fn verify_jolt_fib_e2e_aborts_with_bad_proof() { 
        verify_jolt_aborts_with_bad_proof(&FIB_PROOF, &FIB_ELF)
    }

    #[test]
    fn verify_jolt_sha3_e2e_proof_with_elf_works() { 
        verify_jolt_proof_with_elf_works(&SHA3_PROOF, &SHA3_ELF)
    }

    #[test]
    fn verify_jolt_sha3_e2e_aborts_with_bad_proof() { 
        verify_jolt_aborts_with_bad_proof(&SHA3_PROOF, &SHA3_ELF)
    }
}
