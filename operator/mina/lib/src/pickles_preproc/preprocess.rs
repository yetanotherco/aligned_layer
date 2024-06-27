use crate::{
    state_proof::StateProof,
    type_aliases::{WrapProverCommitments, WrapProverProof, WrapVerifierIndex},
};

pub fn preprocess_state_proof(
    state_proof_json: StateProof,
) -> (WrapVerifierIndex, WrapProverProof) {
    let commitments = WrapProverCommitments {};

    let prover_proof = WrapProverProof {
        commitments,
        proof,
        evals,
        ft_eval1,
        prev_challenges,
    };
    todo!()
}

pub fn compute_prev_challenges() {
    todo!()
}
