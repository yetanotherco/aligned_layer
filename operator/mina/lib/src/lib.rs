use std::sync::Arc;

use kimchi::curve::KimchiCurve;
use kimchi::groupmap::GroupMap;
use kimchi::mina_curves::pasta::{Fp, Pallas, VestaParameters};
use kimchi::mina_poseidon::constants::PlonkSpongeConstantsKimchi;
use kimchi::mina_poseidon::sponge::{DefaultFqSponge, DefaultFrSponge};
use kimchi::poly_commitment::commitment::CommitmentCurve;
use kimchi::verifier::verify;
use kimchi::{mina_curves::pasta::Vesta, poly_commitment::srs::SRS, verifier_index::VerifierIndex};

pub mod openmina_block_verifier;

const MAX_PROOF_SIZE: usize = 10 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 50 * 1024;

type SpongeParams = PlonkSpongeConstantsKimchi;
type BaseSponge = DefaultFqSponge<VestaParameters, SpongeParams>;
type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;

#[no_mangle]
pub extern "C" fn verify_kimchi_proof_ffi(
    proof_bytes: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    pub_input_bytes: &[u8; MAX_PUB_INPUT_SIZE],
    pub_input_len: usize,
) -> bool {
    let proof = if let Ok(proof) = rmp_serde::from_slice(&proof_bytes[..proof_len]) {
        proof
    } else {
        return false;
    };

    let verifier_index = if let Ok(verifier_index) =
        deserialize_kimchi_pub_input(pub_input_bytes[..pub_input_len].to_vec())
    {
        verifier_index
    } else {
        return false;
    };

    let group_map = <Vesta as CommitmentCurve>::Map::setup();

    verify::<Vesta, BaseSponge, ScalarSponge>(&group_map, &verifier_index, &proof, &Vec::new())
        .is_ok()
}

fn deserialize_kimchi_pub_input(
    pub_input_bytes: Vec<u8>,
) -> Result<VerifierIndex<Vesta>, Box<dyn std::error::Error>> {
    let mut verifier_index: VerifierIndex<Vesta> = rmp_serde::from_slice(&pub_input_bytes)?;

    let mut srs = SRS::<Vesta>::create(verifier_index.max_poly_size);
    // add necessary fields to verifier index
    srs.add_lagrange_basis(verifier_index.domain);
    // we only need srs to be embedded in the verifier index, so no need to return it
    verifier_index.srs = Arc::new(srs).into();
    verifier_index.endo = Pallas::endos().0;

    Ok(verifier_index)
}

#[cfg(test)]
mod test {
    use super::*;

    use kimchi::groupmap::GroupMap;
    use kimchi::proof::ProverProof;
    use kimchi::{poly_commitment::commitment::CommitmentCurve, verifier::verify};

    const KIMCHI_PROOF: &[u8] = include_bytes!("../kimchi_ec_add.proof");
    const KIMCHI_VERIFIER_INDEX: &[u8] = include_bytes!("../kimchi_verifier_index.bin");

    #[test]
    fn kimchi_ec_add_proof_verifies() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = KIMCHI_PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(KIMCHI_PROOF);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = KIMCHI_VERIFIER_INDEX.len();
        pub_input_buffer[..pub_input_size].clone_from_slice(KIMCHI_VERIFIER_INDEX);

        let result =
            verify_kimchi_proof_ffi(&proof_buffer, proof_size, &pub_input_buffer, pub_input_size);

        assert!(result)
    }

    #[test]
    fn serialize_deserialize_pub_input_works() {
        let proof: ProverProof<Vesta> = rmp_serde::from_slice(KIMCHI_PROOF)
            .expect("Could not deserialize kimchi proof from file");

        let mut verifier_index: VerifierIndex<Vesta> = rmp_serde::from_slice(KIMCHI_VERIFIER_INDEX)
            .expect("Could not deserialize verifier index");

        let mut srs = SRS::<Vesta>::create(verifier_index.max_poly_size);

        srs.add_lagrange_basis(verifier_index.domain);
        verifier_index.srs = Arc::new(srs.clone()).into();
        verifier_index.endo = Pallas::endos().0;

        // sanity check that the proof verifies with the loaded files
        let group_map = <Vesta as CommitmentCurve>::Map::setup();
        assert!(verify::<Vesta, BaseSponge, ScalarSponge>(
            &group_map,
            &verifier_index,
            &proof,
            &Vec::new(),
        )
        .is_ok());

        // serialize and then deserialize aggregated kimchi pub inputs
        let pub_input_bytes = rmp_serde::to_vec(&verifier_index).unwrap();
        let deserialized_verifier_index = deserialize_kimchi_pub_input(pub_input_bytes).unwrap();
        // verify the proof with the deserialized pub input (verifier index)
        assert!(verify::<Vesta, BaseSponge, ScalarSponge>(
            &group_map,
            &deserialized_verifier_index,
            &proof,
            &Vec::new(),
        )
        .is_ok());
    }
}
