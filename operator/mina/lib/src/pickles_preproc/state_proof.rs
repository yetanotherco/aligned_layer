use kimchi::{
    mina_curves::pasta::{Fp, Fq, Pallas},
    poly_commitment::PolyComm,
};
use o1_utils::FieldHelpers;
use serde::Deserialize;

use super::type_aliases::{WrapPolyComm, WrapScalar};

type DecimalSigned = String;
type HexPointCoordinates = [String; 2];
type HexScalar = String;

#[derive(Deserialize)]
pub struct StateProof {
    pub proof: Proof,
    pub statement: Statement,
}

#[derive(Deserialize)]
pub struct Proof {
    pub bulletproof: Bulletproof,
}

#[derive(Deserialize)]
pub struct Bulletproof {
    pub challenge_polynomial_commitment: HexPointCoordinates,
    pub delta: HexPointCoordinates,
    pub lr: [[HexPointCoordinates; 2]; 15],
    pub z_1: HexScalar,
    pub z_2: HexScalar,
}

#[derive(Deserialize)]
pub struct Commitments {
    pub t_comm: [HexPointCoordinates; 7],
    pub w_comm: [HexPointCoordinates; 15],
    pub z_comm: HexPointCoordinates,
}

#[derive(Deserialize)]
pub struct Evaluations {
    pub coefficients: [HexPointCoordinates; 15],
    pub complete_add_selector: HexPointCoordinates,
    pub emul_selector: HexPointCoordinates,
    pub endomul_scalar_selector: HexPointCoordinates,
    pub generic_selector: HexPointCoordinates,
    pub mul_selector: HexPointCoordinates,
    pub poseidon_selector: HexPointCoordinates,
    pub s: [HexPointCoordinates; 6],
    pub w: [HexPointCoordinates; 15],
    pub z: HexPointCoordinates,
    pub ft_eval1: HexScalar,
}

#[derive(Deserialize)]
pub struct Statement {
    pub messages_for_next_step_proof: MessagesForNextStepProof,
}

#[derive(Deserialize)]
pub struct MessagesForNextStepProof {
    pub challenge_polynomial_commitments: [HexPointCoordinates; 2],
    pub old_bulletproof_challenges: [[BulletproofChallenge; 16]; 2],
}

#[derive(Deserialize)]
pub struct BulletproofChallenge {
    pub prechallenge: Prechallenge,
}

#[derive(Deserialize)]
pub struct Prechallenge {
    // OCaml doesn't support unsigned integers, these should
    // be two u64 limbs but are encoded with a sign.
    // We just need to do a cast to u64.
    pub inner: [DecimalSigned; 2],
}

#[derive(Deserialize)]
pub struct ProofState {
    pub deferred_values: DeferredValues,
    pub messages_for_next_wrap_proof: MessagesForNextWrapProof,
    pub sponge_digest_before_evaluations: [HexScalar; 4],
}

#[derive(Deserialize)]
pub struct DeferredValues {
    pub branch_data: BranchData,
    pub bulletproof_challenges: [BulletproofChallenge; 16],
    pub plonk: Plonk,
}

#[derive(Deserialize)]
pub struct BranchData {
    pub domain_log2: String,
    pub proofs_verified: [String; 1],
}

#[derive(Deserialize)]
pub struct Plonk {
    pub alpha: Prechallenge,
    pub beta: HexPointCoordinates,
    pub feature_flags: FeatureFlags,
    pub gamma: HexPointCoordinates,
    pub zeta: Prechallenge,
}

#[derive(Deserialize)]
pub struct FeatureFlags {
    pub foreign_field_add: bool,
    pub foreign_field_mul: bool,
    pub lookup: bool,
    pub range_check0: bool,
    pub range_check1: bool,
    pub rot: bool,
    pub runtime_tables: bool,
    pub xor: bool,
}

#[derive(Deserialize)]
pub struct MessagesForNextWrapProof {
    pub challenge_polynomial_commitment: HexPointCoordinates,
    pub old_bulletproof_challenges: [[BulletproofChallenge; 16]; 2],
}

impl TryFrom<HexPointCoordinates> for WrapPolyComm {
    type Error = String;

    fn try_from(value: HexPointCoordinates) -> Result<Self, Self::Error> {
        let x = Fp::from_hex(&value[0]).map_err(|err| err.to_string())?;
        let y = Fp::from_hex(&value[1]).map_err(|err| err.to_string())?;
        let p = Pallas::new(x, y, false);
        Ok(WrapPolyComm(PolyComm { elems: vec![p] }))
    }
}

impl TryFrom<HexScalar> for WrapScalar {
    type Error = String;

    fn try_from(value: HexScalar) -> Result<Self, Self::Error> {
        Fq::from_hex(&value)
            .map(WrapScalar)
            .map_err(|err| err.to_string())
    }
}

pub fn parse(proof_json: &serde_json::Value) -> Result<StateProof, String> {
    serde_json::from_value(proof_json.to_owned())
        .map_err(|err| format!("Could not parse proof: {err}"))
}
