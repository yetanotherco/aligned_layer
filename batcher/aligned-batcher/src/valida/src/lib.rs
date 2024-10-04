use valida_basic::BasicMachine;

use p3_baby_bear::BabyBear;

use p3_fri::{FriConfig, TwoAdicFriPcs, TwoAdicFriPcsConfig};
use valida_cpu::MachineWithCpuChip;
use valida_machine::{FixedAdviceProvider, Machine, MachineProof};

use valida_elf::{load_executable_file, Program};
use valida_program::MachineWithProgramChip;
use valida_static_data::MachineWithStaticDataChip;

use p3_challenger::DuplexChallenger;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_field::Field;
use p3_keccak::Keccak256Hash;
use p3_mds::coset_mds::CosetMds;
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_poseidon::Poseidon;
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher32};
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use valida_machine::StarkConfigImpl;
use valida_machine::__internal::p3_commit::ExtensionMmcs;

pub fn verify_valida_proof(code: &[u8], proof: &[u8]) -> bool {
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

    // Run the program
    machine.run(&code, &mut FixedAdviceProvider::empty());

    type Val = BabyBear;
    type Challenge = BinomialExtensionField<Val, 5>;
    type PackedChallenge = BinomialExtensionField<<Val as Field>::Packing, 5>;

    type Mds16 = CosetMds<Val, 16>;
    let mds16 = Mds16::default();

    type Perm16 = Poseidon<Val, Mds16, 16, 5>;
    let mut rng: Pcg64 = Seeder::from("valida seed").make_rng();
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

    let proof: MachineProof<MyConfig> = ciborium::from_reader(proof).expect("wherever");
    let verification_result = machine.verify(&config, &proof);

    verification_result.is_ok()
}
