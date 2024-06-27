use kimchi::{
    mina_curves::pasta::Pallas,
    poly_commitment::{evaluation_proof::OpeningProof, PolyComm},
    proof::{ProverCommitments, ProverProof},
    verifier_index::VerifierIndex,
};

// Wrap circuit specific types
pub type WrapPolyComm = PolyComm<Pallas>;
pub type WrapVerifierIndex = VerifierIndex<Pallas, WrapOpeningProof>;
pub type WrapProverProof = ProverProof<Pallas, WrapOpeningProof>;
pub type WrapProverCommitments = ProverCommitments<Pallas>;
pub type WrapOpeningProof = OpeningProof<Pallas>;
