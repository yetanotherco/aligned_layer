use sp1_core::SP1Verifier;

pub const MAX_PROOF_SIZE: usize = 1024 * 1024;
pub const MAX_ELF_BUFFER_SIZE: usize = 1024 * 1024;

#[no_mangle]
pub extern "C" fn verify_sp1_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    elf_bytes: &[u8; MAX_ELF_BUFFER_SIZE],
    elf_len: usize,
) -> bool {
    let real_elf = &elf_bytes[0..elf_len];

    if let Ok(proof) = bincode::deserialize(&proof_bytes[..proof_len]) {
        return SP1Verifier::verify(real_elf, &proof).is_ok();
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/sp1/sp1_fibonacci.proof");
    const ELF: &[u8] = include_bytes!("../elf/riscv32im-succinct-zkvm-elf");

    #[test]
    fn verify_sp1_proof_with_elf_works() {
        let mut proof_buffer = [0u8; MAX_PROOF_SIZE];
        let proof_size = PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(PROOF);

        let mut elf_buffer = [0u8; MAX_ELF_BUFFER_SIZE];
        let elf_size = ELF.len();
        elf_buffer[..elf_size].clone_from_slice(ELF);

        let result = verify_sp1_proof_ffi(&proof_buffer, proof_size, &elf_buffer, elf_size);
        assert!(result)
    }

    #[test]
    fn verify_sp1_aborts_with_bad_proof() {
        let mut proof_buffer = [42u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(PROOF);

        let mut elf_buffer = [0u8; MAX_ELF_BUFFER_SIZE];
        let elf_size = ELF.len();
        elf_buffer[..elf_size].clone_from_slice(ELF);

        let result = verify_sp1_proof_ffi(&proof_buffer, proof_size - 1, &elf_buffer, elf_size);
        assert!(!result)
    }
}
