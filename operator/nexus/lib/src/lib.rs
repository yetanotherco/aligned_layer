use std::slice;
use nexus_api::{
    config::vm::{ProverImpl, VmConfig},
    nvm::{self, memory::MerkleTrie, NexusVM},
    prover::{self},
};
use nexus_config::vm::NovaImpl;

const CONFIG: VmConfig = VmConfig {
    k: 1,
    prover: ProverImpl::Nova(NovaImpl::Sequential),
};

#[no_mangle]
pub extern "C" fn verify_nexus_proof_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    params_bytes: *const u8,
    params_len: u32,
    elf_bytes: *const u8,
    elf_len: u32,
) -> bool {
    let proof_bytes = unsafe {
        assert!(!proof_bytes.is_null());
        slice::from_raw_parts(proof_bytes, proof_len as usize)
    };

    let params_bytes = unsafe {
        assert!(!params_bytes.is_null());
        slice::from_raw_parts(params_bytes, params_len as usize)
    };

    let input_bytes = unsafe {
        assert!(!input_bytes.is_null());
        slice::from_raw_parts(input_bytes, input_len as usize)
    };

    let elf_bytes = unsafe {
        assert!(!elf_bytes.is_null());
        slice::from_raw_parts(elf_bytes, elf_len as usize)
    };

    let mut vm: NexusVM<MerkleTrie> =
        nvm::interactive::parse_elf(&elf_bytes).expect("error loading and parsing RISC-V instruction");

    vm.syscalls.set_input(&[0x06]);

    let trace = nvm::interactive::trace(
        &mut vm,
        CONFIG.k,
        matches!(CONFIG.prover, ProverImpl::Nova(NovaImpl::Parallel)),
    )
    .expect("error generating execution trace");

    let proof = prover::prove::prove_seq(&public_params, trace).expect("error proving execution");

    proof
        .verify(&public_params, proof.step_num() as _)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/nexus.proof");
    const PARAMS: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/nexus.params");
    const INPUT: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/nexus.input");
    const ELF: &[u8] =
        include_bytes!("../../../../task_sender/test_examples/nexus/fib/nexus.elf");

    #[test]
    fn verify_nexus_proof_with_elf_works() {
        let proof_bytes = PROOF.as_ptr();
        let params_bytes = PARAMS.as_ptr();
        let input_bytes = INPUT.as_ptr();
        let elf_bytes = ELF.as_ptr();

        let result = verify_nexus_proof_ffi(proof_bytes, PROOF.len() as u32, params_bytes, PARAMS.len() as u32, input_bytes, INPUT.len() as u32, elf_bytes, ELF.len() as u32);
        assert!(result)
    }

    #[test]
    fn verify_nexus_aborts_with_bad_proof() {
        let proof_bytes = PROOF.as_ptr();
        let params_bytes = PARAMS.as_ptr();
        let input_bytes = INPUT.as_ptr();
        let elf_bytes = ELF.as_ptr();

        let result = verify_nexus_proof_ffi(proof_bytes, (PROOF.len() - 1) as u32, PARAMS.len() as u32, PARAMS.len() as u32, input_bytes, INPUT.len() as u32, elf_bytes, ELF.len() as u32);
        assert!(!result)
    }
}
