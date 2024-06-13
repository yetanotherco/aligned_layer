use jolt_core::{
    jolt::vm::{Jolt, JoltPreprocessing, rv32i_vm::{RV32IHyraxProof, RV32IJoltVM}},
    poly::commitment::hyrax::HyraxScheme
};
use tracer::decode;
use ark_serialize::CanonicalDeserialize;
use ark_bn254::{Fr, G1Projective};
use std::slice;

#[no_mangle]
pub extern "C" fn verify_jolt_proof_ffi(
    proof_buffer: *const u8,
    proof_len: u32,
    elf_buffer: *const u8,
    elf_len: u32,
) -> bool {
    let proof_bytes = unsafe {
        assert!(!proof_buffer.is_null());
        slice::from_raw_parts(proof_buffer, proof_len as usize)
    };

    let elf_bytes = unsafe {
        assert!(!elf_buffer.is_null());
        slice::from_raw_parts(elf_buffer, elf_len as usize)
    };

    if let Ok(jolt_proof) = RV32IHyraxProof::deserialize_compressed(&*proof_bytes) {
        // Add public inputs...
        //TODO: check if we need to load function and function args... These should be serialized in the VK
        let (bytecode, memory_init) = decode(&elf_bytes);

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
        include_bytes!("../../../../task_sender/test_examples/jolt/fibonacci/fibonacci-guest.proof");
    const FIB_ELF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/fibonacci/elf/fibonacci-guest.elf");

    // Sha3
    const SHA3_PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/sha3-ex/sha3-guest.proof");
    const SHA3_ELF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/jolt/sha3-ex/elf/sha3-guest.elf");

    fn verify_jolt_proof_with_elf_works(proof: &[u8], elf: &[u8]) {
        let proof_len = proof.len();

        let elf_len = elf.len();

        let result = verify_jolt_proof_ffi(proof.as_ptr(), proof_len as u32, elf.as_ptr(), elf_len as u32);
        assert!(result)
    }

    fn verify_jolt_aborts_with_bad_proof(proof: &[u8], elf: &[u8]) {
        let mut proof_buffer = [42u8; 4 * 1024 * 1024];
        let proof_len = proof.len();
        proof_buffer[..proof_len].clone_from_slice(proof);

        let elf_len = elf.len();

        let result = verify_jolt_proof_ffi(proof_buffer.as_ptr(), (proof_len - 1) as u32, elf.as_ptr(), elf_len as u32);
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
