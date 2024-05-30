use std::slice;
use sp1_sdk::ProverClient;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROVER_CLIENT: ProverClient = ProverClient::new();
}

#[no_mangle]
pub extern "C" fn verify_sp1_proof_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    elf_bytes: *const u8,
    elf_len: u32,
) -> bool {
    println!("Reading proof of length: {:?}", proof_len);
    let proof_bytes = unsafe {
        assert!(!proof_bytes.is_null());

        slice::from_raw_parts(proof_bytes, proof_len as usize)
    };

    println!("Reading elf of length: {:?}", elf_len);
    let elf_bytes = unsafe {
        assert!(!elf_bytes.is_null());

        slice::from_raw_parts(elf_bytes, elf_len as usize)
    };

    println!("Deserializing proof");

    if let Ok(proof) = bincode::deserialize(proof_bytes) {
        println!("Deserialized proof");
        let (_pk, vk) = PROVER_CLIENT.setup(elf_bytes);

        println!("Verifying proof");
        return PROVER_CLIENT.verify_compressed(&proof, &vk).is_ok();
    }

    println!("Failed to deserialize proof");

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/sp1/fibonacci_proof_generator/script/sp1_fibonacci.proof");
    const ELF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/sp1/fibonacci_proof_generator/program/elf/riscv32im-succinct-zkvm-elf");

    #[test]
    fn verify_sp1_proof_with_elf_works() {
        let proof_bytes = PROOF.as_ptr();
        println!("actual proof len: {:?}", PROOF.len());
        let elf_bytes = ELF.as_ptr();
        println!("actual elf len: {:?}", ELF.len());

        let result = verify_sp1_proof_ffi(proof_bytes, PROOF.len() as u32, elf_bytes, ELF.len() as u32);
        assert!(result)
    }

    #[test]
    fn verify_sp1_aborts_with_bad_proof() {
        let mut proof_buffer = [42u8; PROOF.len()];
        proof_buffer.clone_from_slice(PROOF);

        let mut elf_buffer = [42u8; ELF.len()];
        elf_buffer.clone_from_slice(ELF);

        let proof_bytes = proof_buffer.as_ptr();
        let elf_bytes = elf_buffer.as_ptr();

        let result = verify_sp1_proof_ffi(proof_bytes, (PROOF.len() - 1) as u32, elf_bytes, ELF.len() as u32);
        assert!(!result)
    }
}
