use std::array;

use ark_ff::BigInteger256;
use kimchi::{
    curve::KimchiCurve,
    mina_curves::pasta::{Fp, Vesta},
    mina_poseidon::{constants::PlonkSpongeConstantsKimchi, sponge::DefaultFrSponge},
    plonk_sponge::FrSponge,
};
use o1_utils::FieldHelpers;

use super::state_proof::{ProofState, Statement};

pub struct WrapSponge(pub DefaultFrSponge<Fp, PlonkSpongeConstantsKimchi>);

impl WrapSponge {
    pub fn new(proof_state: ProofState) -> Self {
        let mut ret = DefaultFrSponge::new(Vesta::sponge_params());
        let digest = Fp::new(BigInteger256::new(
            proof_state.sponge_digest_before_evaluations,
        ));
        ret.absorb(&digest);

        Self(ret)
    }
}
