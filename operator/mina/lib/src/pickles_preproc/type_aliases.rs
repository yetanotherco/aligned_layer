use kimchi::{
    mina_curves::pasta::{Fq, Pallas},
    poly_commitment::evaluation_proof::OpeningProof,
    proof::{PointEvaluations, ProofEvaluations, ProverCommitments, ProverProof},
    verifier_index::VerifierIndex,
};

// Wrap circuit specific types

pub struct WrapECPoint(pub Pallas);
pub struct WrapScalar(pub Fq);
pub struct WrapPointEvaluations(pub PointEvaluations<Vec<Fq>>);

pub type WrapVerifierIndex = VerifierIndex<Pallas, WrapOpeningProof>;

pub type WrapProverProof = ProverProof<Pallas, WrapOpeningProof>;
pub type WrapProverCommitments = ProverCommitments<Pallas>;
pub type WrapOpeningProof = OpeningProof<Pallas>;
pub type WrapProofEvaluations = ProofEvaluations<PointEvaluations<Vec<Fq>>>;
