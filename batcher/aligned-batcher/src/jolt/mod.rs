use jolt_core::{
    jolt::vm::{Jolt, JoltPreprocessing, rv32i_vm::RV32IJoltVM},
    poly::commitment::hyrax::HyraxScheme
};
use jolt_sdk::host_utils::RV32IHyraxProof;
use tracer::decode;
use ark_serialize::CanonicalDeserialize;
use ark_bn254::{Fr, G1Projective};
use log::{debug, warn};

pub fn verify_jolt_proof(proof: &[u8], elf: &[u8]) -> bool {
    debug!("Verifying Jolt proof");
    if let Ok(jolt_proof) = RV32IHyraxProof::deserialize_compressed(proof) {
            let (bytecode, memory_init) = decode(&elf);

            let preprocessing: JoltPreprocessing<Fr, HyraxScheme<G1Projective>> =
                RV32IJoltVM::preprocess(bytecode.clone(), memory_init, 1 << 20, 1 << 20, 1 << 24);

            let res = RV32IJoltVM::verify(preprocessing, jolt_proof.proof, jolt_proof.commitments).is_ok();
            debug!("Jolt proof is valid: {}", res);
            if res {
                return true;
            }
    }

    warn!("Failed to decode JOLT proof");

    false
}