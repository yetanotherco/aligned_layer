use lazy_static::lazy_static;
use log::error;
use sp1_sdk::ProverClient;

lazy_static! {
    static ref PROVER_CLIENT: ProverClient = ProverClient::new();
}

fn inner_verify_sp1_proof_old_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    elf_bytes: *const u8,
    elf_len: u32,
) -> bool {
    if proof_bytes.is_null() || elf_bytes.is_null() {
        error!("Input buffer null");
        return false;
    }

    if proof_len == 0 || elf_len == 0 {
        error!("Input buffer length zero size");
        return false;
    }

    let proof_bytes = unsafe { std::slice::from_raw_parts(proof_bytes, proof_len as usize) };

    let elf_bytes = unsafe { std::slice::from_raw_parts(elf_bytes, elf_len as usize) };

    if let Ok(proof) = bincode::deserialize(proof_bytes) {
        let (_pk, vk) = PROVER_CLIENT.setup(elf_bytes);
        return PROVER_CLIENT.verify(&proof, &vk).is_ok();
    }

    false
}

#[no_mangle]
pub extern "C" fn verify_sp1_proof_old_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    elf_bytes: *const u8,
    elf_len: u32,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        inner_verify_sp1_proof_old_ffi(proof_bytes, proof_len, elf_bytes, elf_len)
    });

    match result {
        Ok(v) => v as i32,
        Err(_) => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] = include_bytes!("../../../../scripts/test_files/sp1/sp1_fibonacci.proof");
    const ELF: &[u8] = include_bytes!("../../../../scripts/test_files/sp1/sp1_fibonacci.elf");

    #[test]
    fn verify_sp1_proof_with_elf_works() {
        let proof_bytes = PROOF.as_ptr();
        let elf_bytes = ELF.as_ptr();

        let result =
            verify_sp1_proof_old_ffi(proof_bytes, PROOF.len() as u32, elf_bytes, ELF.len() as u32);
        assert_eq!(result, 1)
    }

    #[test]
    fn verify_sp1_aborts_with_bad_proof() {
        let proof_bytes = PROOF.as_ptr();
        let elf_bytes = ELF.as_ptr();

        let result = verify_sp1_proof_old_ffi(
            proof_bytes,
            (PROOF.len() - 1) as u32,
            elf_bytes,
            ELF.len() as u32,
        );
        assert_eq!(result, 0)
    }
}
