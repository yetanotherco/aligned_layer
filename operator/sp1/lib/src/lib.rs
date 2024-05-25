use sp1_sdk::ProverClient;
use lazy_static::lazy_static;
pub const MAX_PROOF_SIZE: usize = 2 * 1024 * 1024;
pub const MAX_ELF_BUFFER_SIZE: usize = 1024 * 1024;

lazy_static! {
    static ref PROVER_CLIENT: ProverClient = ProverClient::new();
}

#[no_mangle]
pub extern "C" fn verify_sp1_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: u32,
    elf_bytes: &[u8; MAX_ELF_BUFFER_SIZE],
    elf_len: u32,
) -> bool {
    let real_elf = &elf_bytes[0..(elf_len as usize)];

    if let Ok(proof) = bincode::deserialize(&proof_bytes[..(proof_len as usize)]) {
        let (_pk, vk) = PROVER_CLIENT.setup(real_elf);
        return PROVER_CLIENT.verify(&proof, &vk).is_ok();
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
    fn verify_sp1_proof_with_elf_works() {
        let mut proof_buffer = [0u8; MAX_PROOF_SIZE];
        let proof_size = PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(PROOF);

        let mut elf_buffer = [0u8; MAX_ELF_BUFFER_SIZE];
        let elf_size = ELF.len();
        elf_buffer[..elf_size].clone_from_slice(ELF);

        let result = verify_sp1_proof_ffi(&proof_buffer, proof_size as u32, &elf_buffer, elf_size as u32);
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

        let result = verify_sp1_proof_ffi(&proof_buffer, (proof_size - 1) as u32, &elf_buffer, elf_size as u32);
        assert!(!result)
    }
}
