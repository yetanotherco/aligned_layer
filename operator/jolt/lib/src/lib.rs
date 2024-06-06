use lazy_static::lazy_static;
use jolt::jolt_core::jolt::vm::{JoltProof, rv32i_vm::{Jolt, RV32IJoltVM}};
use jolt::jolt_core::poly::commitment::commitment_scheme::CommitmentScheme;
use jolt::jolt_core::poly::commitment::hyperkzg::HyperKZG;
use jolt::jolt_core::poly::commitment::hyrax::HyraxScheme;
use jolt::tracer;

#[no_mangle]
pub extern "C" fn verify_jolt_proof_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    elf_bytes: *const u8,
    elf_len: u32,
    commitment_bytes: *const u8,
    commitment_byte_len: u32,
) -> bool {
    if let Ok(jolt_proof) = bincode::deserialize(&proof_bytes[..proof_len as usize]) {
        if let ok(jolt_commitments) = bincode::deserialize(&commitment_bytes[..commitment_byte_len as usize]) {
            let (bytecode, memory_init) = tracer::decode(&*elf_bytes[..elf_len as usize]);

            let preprocessing =
                RV32IJoltVM::preprocess(bytecode.clone(), memory_init, 1 << 20, 1 << 20, 1 << 20);

            let verification_result = RV32IJoltVM::verify(preprocessing, jolt_proof, jolt_commitments);
            verification_result.is_ok(),
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/sp1/sp1_fibonacci.proof");
    const ELF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/sp1/elf/riscv32im-succinct-zkvm-elf");

    #[test]
    fn verify_jolt_proof_with_elf_works() {
        let mut proof_buffer = [0u8; MAX_PROOF_SIZE];
        let proof_size = PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(PROOF);

        let mut elf_buffer = [0u8; MAX_ELF_BUFFER_SIZE];
        let elf_size = ELF.len();
        elf_buffer[..elf_size].clone_from_slice(ELF);

        let result = verify_jolt_proof_ffi(&proof_buffer, proof_size, &elf_buffer, elf_size);
        assert!(result)
    }

    #[test]
    fn verify_jolt_aborts_with_bad_proof() {
        let mut proof_buffer = [42u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(PROOF);

        let mut elf_buffer = [0u8; MAX_ELF_BUFFER_SIZE];
        let elf_size = ELF.len();
        elf_buffer[..elf_size].clone_from_slice(ELF);

        let result = verify_jolt_proof_ffi(&proof_buffer, proof_size - 1, &elf_buffer, elf_size);
        assert!(!result)
    }
}
