use mina_p2p_messages::v2::{
    MinaBaseProofStableV2, PicklesProofProofsVerified2ReprStableV2,
    PicklesProofProofsVerified2ReprStableV2PrevEvals,
    PicklesProofProofsVerified2ReprStableV2Statement, PicklesWrapWireProofCommitmentsStableV1,
    PicklesWrapWireProofEvaluationsStableV1, PicklesWrapWireProofStableV1,
    PicklesWrapWireProofStableV1Bulletproof,
};
use serde::Deserialize;

pub const WRAP_PREV_CHALLENGES: usize = 2;
pub const WRAP_SCALARS_PER_CHALLENGE: usize = 15;

pub type DecimalSigned = String;
pub type HexScalar = String;
pub type HexPointCoordinates = [String; 2];
pub type HexPointEvaluations = [String; 2];

#[derive(Deserialize)]
pub struct StateProof {
    pub proof: Proof,
    pub statement: Statement,
}

#[derive(Deserialize)]
pub struct Proof {
    pub commitments: Commitments,
    pub evaluations: Evaluations,
    pub ft_eval1: HexScalar,
    pub bulletproof: Bulletproof,
}

#[derive(Deserialize)]
pub struct Bulletproof {
    pub challenge_polynomial_commitment: HexPointCoordinates,
    pub delta: HexPointCoordinates,
    pub lr: Vec<(HexPointCoordinates, HexPointCoordinates)>,
    pub z_1: HexScalar,
    pub z_2: HexScalar,
}

#[derive(Deserialize)]
pub struct Commitments {
    pub w_comm: [HexPointCoordinates; 15],
    pub z_comm: HexPointCoordinates,
    pub t_comm: Vec<HexPointCoordinates>,
}

#[derive(Deserialize)]
pub struct Evaluations {
    pub coefficients: [HexPointEvaluations; 15],
    pub complete_add_selector: HexPointEvaluations,
    pub emul_selector: HexPointEvaluations,
    pub endomul_scalar_selector: HexPointEvaluations,
    pub generic_selector: HexPointEvaluations,
    pub mul_selector: HexPointEvaluations,
    pub poseidon_selector: HexPointEvaluations,
    pub s: [HexPointEvaluations; 6],
    pub w: [HexPointEvaluations; 15],
    pub z: HexPointEvaluations,
}

#[derive(Deserialize)]
pub struct Statement {
    pub proof_state: ProofState,
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
    pub old_bulletproof_challenges:
        [[BulletproofChallenge; WRAP_SCALARS_PER_CHALLENGE]; WRAP_PREV_CHALLENGES],
}

pub fn parse_json(mina_state_proof_vk_query_str: &str) -> Result<StateProof, String> {
    let mina_state_proof_vk_query: serde_json::Map<String, serde_json::Value> =
        serde_json::from_str(mina_state_proof_vk_query_str)
            .map_err(|err| format!("Could not parse mina state proof vk query: {err}"))?;
    let protocol_state_proof_json = mina_state_proof_vk_query
            .get("data")
            .and_then(|d| d.get("bestChain"))
            .and_then(|d| d.get(0))
            .and_then(|d| d.get("protocolStateProof"))
            .and_then(|d| d.get("json"))
            .ok_or("Could not parse protocol state proof: JSON structure upto protocolStateProof is unexpected")?;

    serde_json::from_value(protocol_state_proof_json.to_owned())
        .map_err(|err| format!("Could not parse mina state proof: {err}"))
}

pub fn parse_state_proof(state_proof: StateProof) -> Result<MinaBaseProofStableV2, String> {
    // proof fields
    let commitments = PicklesWrapWireProofCommitmentsStableV1 {
        w_comm: (),
        z_comm: (),
        t_comm: (),
    };
    let evaluations = PicklesWrapWireProofEvaluationsStableV1 {
        w: (),
        coefficients: (),
        z: (),
        s: (),
        generic_selector: (),
        poseidon_selector: (),
        complete_add_selector: (),
        mul_selector: (),
        emul_selector: (),
        endomul_scalar_selector: (),
    };
    //let ft_eval1 = bigint
    let bulletproof = PicklesWrapWireProofStableV1Bulletproof {
        lr: (),
        z_1: (),
        z_2: (),
        delta: (),
        challenge_polynomial_commitment: (),
    };

    // protocol_state_proof fields
    let statement = PicklesProofProofsVerified2ReprStableV2Statement {
        proof_state: (),
        messages_for_next_step_proof: (),
    };
    let prev_evals = PicklesProofProofsVerified2ReprStableV2PrevEvals {
        evals: (),
        ft_eval1: (),
    };
    let proof = PicklesWrapWireProofStableV1 {
        commitments: (),
        evaluations: (),
        ft_eval1: (),
        bulletproof: (),
    };

    let protocol_state_proof = MinaBaseProofStableV2(PicklesProofProofsVerified2ReprStableV2 {
        statement: (),
        prev_evals: (),
        proof: (),
    });

    protocol_state_proof
}

#[cfg(test)]
mod tests {
    use super::parse_json;

    const MINA_STATE_PROOF_VK_QUERY: &str = include_str!(
        "../../../../../batcher/aligned/test_files/mina/mina_state_proof_vk_query.json"
    );

    #[test]
    fn parse_protocol_state_proof() {
        parse_json(MINA_STATE_PROOF_VK_QUERY).unwrap();
    }
}
