use kimchi::{circuits::wires::COLUMNS, mina_curves::pasta::Pallas, poly_commitment::PolyComm};

use crate::pickles_preproc::{
    state_proof::{Bulletproof, Commitments},
    type_aliases::{WrapECPoint, WrapOpeningProof, WrapProverCommitments, WrapScalar},
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

    let _commitments = WrapProverCommitments {
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

    let _opening_proof = WrapOpeningProof {
        sg,
        delta,
        lr,
        z1,
        z2,
    };

    /*
        let prover_proof = WrapProverProof {
            commitments,
            proof,
            evals,
            ft_eval1,
            prev_challenges,
        };
    */

    todo!()
}

pub fn compute_prev_challenges() {
    todo!()
}
