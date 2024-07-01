use kimchi::{circuits::wires::COLUMNS, mina_curves::pasta::Pallas, poly_commitment::PolyComm};

use crate::pickles_preproc::{
    state_proof::Commitments,
    type_aliases::{WrapECPoint, WrapProverCommitments},
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
