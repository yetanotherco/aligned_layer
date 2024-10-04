use log::error;

use p3_baby_bear::BabyBear;
use p3_challenger::DuplexChallenger;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_field::Field;
use p3_fri::{FriConfig, TwoAdicFriPcs, TwoAdicFriPcsConfig};
use p3_keccak::Keccak256Hash;
use p3_mds::coset_mds::CosetMds;
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_poseidon::Poseidon;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher32};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use valida_basic::BasicMachine;
use valida_cpu::MachineWithCpuChip;
use valida_elf::{load_executable_file, Program};
use valida_machine::StarkConfigImpl;
use valida_machine::__internal::p3_commit::ExtensionMmcs;
use valida_machine::{FixedAdviceProvider, GlobalAdviceProvider, Machine, MachineProof};
use valida_program::MachineWithProgramChip;
use valida_static_data::MachineWithStaticDataChip;

#[no_mangle]
pub extern "C" fn verify_valida_proof_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    program_bytes: *const u8,
    program_len: u32,
) -> bool {
    if proof_bytes.is_null() || program_bytes.is_null() {
        error!("Input buffer null");
        return false;
    }

    if proof_len == 0 || program_len == 0 {
        error!("Input buffer length zero size");
        return false;
    }

    let proof = unsafe { std::slice::from_raw_parts(proof_bytes, proof_len as usize) };
    let code = unsafe { std::slice::from_raw_parts(program_bytes, program_len as usize) };

    let mut machine = BasicMachine::<BabyBear>::default();
    let Program {
        code,
        data,
        initial_program_counter,
    } = load_executable_file(code.to_vec());
    machine.program_mut().set_program_rom(&code);
    machine.cpu_mut().fp = 16777216;
    machine.cpu_mut().pc = initial_program_counter;
    machine.cpu_mut().save_register_state();
    machine.static_data_mut().load(data);

    machine.run(&code, &mut FixedAdviceProvider::empty());

    type Val = BabyBear;
    type Challenge = BinomialExtensionField<Val, 5>;
    type PackedChallenge = BinomialExtensionField<<Val as Field>::Packing, 5>;

    type Mds16 = CosetMds<Val, 16>;
    let mds16 = Mds16::default();

    type Perm16 = Poseidon<Val, Mds16, 16, 5>;
    let mut rng: Pcg64 = Seeder::from("validia seed").make_rng();
    let perm16 = Perm16::new_from_rng(4, 22, mds16, &mut rng);

    type MyHash = SerializingHasher32<Keccak256Hash>;
    let hash = MyHash::new(Keccak256Hash {});

    type MyCompress = CompressionFunctionFromHasher<Val, MyHash, 2, 8>;
    let compress = MyCompress::new(hash);

    type ValMmcs = FieldMerkleTreeMmcs<Val, MyHash, MyCompress, 8>;
    let val_mmcs = ValMmcs::new(hash, compress);

    type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

    type Dft = Radix2DitParallel;
    let dft = Dft::default();

    type Challenger = DuplexChallenger<Val, Perm16, 16>;

    type MyFriConfig = TwoAdicFriPcsConfig<Val, Challenge, Challenger, Dft, ValMmcs, ChallengeMmcs>;
    let fri_config = FriConfig {
        log_blowup: 1,
        num_queries: 40,
        proof_of_work_bits: 8,
        mmcs: challenge_mmcs,
    };

    type Pcs = TwoAdicFriPcs<MyFriConfig>;
    type MyConfig = StarkConfigImpl<Val, Challenge, PackedChallenge, Pcs, Challenger>;

    let pcs = Pcs::new(fri_config, dft, val_mmcs);

    let challenger = Challenger::new(perm16);
    let config = MyConfig::new(pcs, challenger);

    let Ok(proof): Result<MachineProof<MyConfig>, _> = ciborium::from_reader(proof) else {
        return false;
    };
    let verification_result = machine.verify(&config, &proof);

    verification_result.is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROOF: &[u8] =
        include_bytes!("../../../../scripts/test_files/valida/cat/fibonacci.proof");
    const PROGRAM_CODE: &[u8] =
        include_bytes!("../../../../scripts/test_files/valida/cat/fibonacci.bin");

    #[test]
    fn verify_valida_proof_with_program_code_works() {
        let proof = PROOF.as_ptr();
        let program = PROGRAM_CODE.as_ptr();

        let result = verify_valida_proof_ffi(
            proof,
            PROOF.len() as u32,
            program,
            PROGRAM_CODE.len() as u32,
        );
        assert!(result)
    }

    #[test]
    fn verify_valida_aborts_with_bad_proof() {
        let proof = PROOF.as_ptr();
        let program = PROGRAM_CODE.as_ptr();

        let result = verify_valida_proof_ffi(
            proof,
            (PROOF.len() - 1) as u32,
            program,
            PROGRAM_CODE.len() as u32,
        );
        assert!(!result)
    }

    #[test]
    fn verify_valida_input_valid() {
        let proof = PROOF.as_ptr();
        let program = PROGRAM_CODE.as_ptr();

        let result = verify_valida_proof_ffi(proof, (PROOF.len() - 1) as u32, program, 0);
        assert!(!result)
    }
}
