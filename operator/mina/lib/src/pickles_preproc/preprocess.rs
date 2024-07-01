use kimchi::{
    circuits::wires::{COLUMNS, PERMUTS},
    mina_curves::pasta::{Fq, Pallas},
    poly_commitment::PolyComm,
    proof::{PointEvaluations, RecursionChallenge},
};

use crate::pickles_preproc::{
    state_proof::{Bulletproof, Commitments, Evaluations},
    type_aliases::{
        WrapECPoint, WrapOpeningProof, WrapPointEvaluations, WrapProofEvaluations,
        WrapProverCommitments, WrapScalar,
    },
};

use super::{
    state_proof::StateProof,
    type_aliases::{WrapProverProof, WrapVerifierIndex},
};

pub fn deserialize_state_proof(
    state_proof: StateProof,
) -> Result<(WrapVerifierIndex, WrapProverProof), String> {
    let Commitments {
        w_comm: hex_w_comm,
        z_comm: hex_z_comm,
        t_comm: hex_t_comm,
    } = state_proof.proof.commitments;

    // w_comm are single-point commitments
    let mut w_comm: [PolyComm<Pallas>; COLUMNS] = std::array::from_fn(|_| PolyComm {
        elems: Vec::with_capacity(1),
    });
    for (hex_point, comm) in hex_w_comm.into_iter().zip(w_comm.iter_mut()) {
        comm.elems.push(WrapECPoint::try_from(hex_point)?.0);
    }

    // z_comm is a single-point commitment
    let z_comm = PolyComm {
        elems: vec![WrapECPoint::try_from(hex_z_comm)?.0],
    };
    // t_comm is a multi-point commitment
    let t_comm = PolyComm {
        elems: hex_t_comm
            .into_iter()
            .map(|hex_point| WrapECPoint::try_from(hex_point).map(|point| point.0))
            .collect::<Result<_, _>>()?,
    };
    let lookup = None;

    let commitments = WrapProverCommitments {
        w_comm,
        z_comm,
        t_comm,
        lookup,
    };

    let Bulletproof {
        challenge_polynomial_commitment: hex_sg,
        delta: hex_delta,
        lr: hex_lr,
        z_1: hex_z_1,
        z_2: hex_z_2,
    } = state_proof.proof.bulletproof;
    let sg = WrapECPoint::try_from(hex_sg)?.0;
    let delta = WrapECPoint::try_from(hex_delta)?.0;
    let lr = hex_lr
        .into_iter()
        .map(|(hex_p1, hex_p2)| -> Result<(Pallas, Pallas), String> {
            let p1 = WrapECPoint::try_from(hex_p1)?.0;
            let p2 = WrapECPoint::try_from(hex_p2)?.0;
            Ok((p1, p2))
        })
        .collect::<Result<_, _>>()?;
    let z1 = WrapScalar::try_from(hex_z_1)?.0;
    let z2 = WrapScalar::try_from(hex_z_2)?.0;

    let proof = WrapOpeningProof {
        sg,
        delta,
        lr,
        z1,
        z2,
    };

    let Evaluations {
        coefficients: hex_coefficients,
        complete_add_selector: hex_complete_add_selector,
        emul_selector: hex_emul_selector,
        endomul_scalar_selector: hex_endomul_scalar_selector,
        generic_selector: hex_generic_selector,
        mul_selector: hex_mul_selector,
        poseidon_selector: hex_poseidon_selector,
        s: hex_s,
        w: hex_w,
        z: hex_z,
    } = state_proof.proof.evaluations;

    let mut w: [PointEvaluations<Vec<Fq>>; COLUMNS] = std::array::from_fn(|_| PointEvaluations {
        zeta: Vec::with_capacity(1),
        zeta_omega: Vec::with_capacity(1),
    });
    for (hex_eval, eval) in hex_w.into_iter().zip(w.iter_mut()) {
        *eval = WrapPointEvaluations::try_from(hex_eval)?.0;
    }

    let z = WrapPointEvaluations::try_from(hex_z)?.0;

    let mut s: [PointEvaluations<Vec<Fq>>; PERMUTS - 1] =
        std::array::from_fn(|_| PointEvaluations {
            zeta: Vec::with_capacity(1),
            zeta_omega: Vec::with_capacity(1),
        });
    for (hex_eval, eval) in hex_s.into_iter().zip(s.iter_mut()) {
        *eval = WrapPointEvaluations::try_from(hex_eval)?.0;
    }

    let mut coefficients: [PointEvaluations<Vec<Fq>>; COLUMNS] =
        std::array::from_fn(|_| PointEvaluations {
            zeta: Vec::with_capacity(1),
            zeta_omega: Vec::with_capacity(1),
        });
    for (hex_eval, eval) in hex_coefficients.into_iter().zip(coefficients.iter_mut()) {
        *eval = WrapPointEvaluations::try_from(hex_eval)?.0;
    }

    let generic_selector = WrapPointEvaluations::try_from(hex_generic_selector)?.0;
    let poseidon_selector = WrapPointEvaluations::try_from(hex_poseidon_selector)?.0;
    let complete_add_selector = WrapPointEvaluations::try_from(hex_complete_add_selector)?.0;
    let mul_selector = WrapPointEvaluations::try_from(hex_mul_selector)?.0;
    let emul_selector = WrapPointEvaluations::try_from(hex_emul_selector)?.0;
    let endomul_scalar_selector = WrapPointEvaluations::try_from(hex_endomul_scalar_selector)?.0;

    let public = None; // TODO: Calculate public poly evaluations

    let evals = WrapProofEvaluations {
        public,
        w,
        z,
        s,
        coefficients,
        generic_selector,
        poseidon_selector,
        complete_add_selector,
        mul_selector,
        emul_selector,
        endomul_scalar_selector,
        range_check0_selector: None,
        range_check1_selector: None,
        foreign_field_add_selector: None,
        foreign_field_mul_selector: None,
        xor_selector: None,
        rot_selector: None,
        lookup_aggregation: None,
        lookup_table: None,
        lookup_sorted: std::array::from_fn(|_| None),
        runtime_lookup_table: None,
        runtime_lookup_table_selector: None,
        xor_lookup_selector: None,
        lookup_gate_lookup_selector: None,
        range_check_lookup_selector: None,
        foreign_field_mul_lookup_selector: None,
    };

    let ft_eval1 = WrapScalar::try_from(state_proof.proof.ft_eval1)?.0;

    // TODO: Calculate prev_challenges
    let prev_challenges = vec![RecursionChallenge {
        chals: Vec::new(),
        comm: PolyComm {
            elems: Vec::<Pallas>::new(),
        },
    }];

    let _prover_proof = WrapProverProof {
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
