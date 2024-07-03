use kimchi::{
    circuits::wires::{COLUMNS, PERMUTS},
    mina_curves::pasta::{Fq, Pallas},
    mina_poseidon::sponge::ScalarChallenge,
    poly_commitment::{srs::endos, PolyComm},
    proof::{PointEvaluations, RecursionChallenge},
};

use crate::pickles_preproc::{
    state_proof::{Bulletproof, Commitments, Evaluations, WRAP_PREV_CHALLENGES},
    type_aliases::{
        WrapECPoint, WrapOpeningProof, WrapPointEvaluations, WrapProofEvaluations,
        WrapProverCommitments, WrapScalar,
    },
};

use super::{
    state_proof::{
        BulletproofChallenge, HexPointCoordinates, StateProof, WRAP_SCALARS_PER_CHALLENGE,
    },
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

    let prev_challenges = compute_prev_challenges(
        state_proof
            .statement
            .proof_state
            .messages_for_next_wrap_proof
            .old_bulletproof_challenges,
        state_proof
            .statement
            .messages_for_next_step_proof
            .challenge_polynomial_commitments,
    )?;

    let _prover_proof = WrapProverProof {
        commitments,
        proof,
        evals,
        ft_eval1,
        prev_challenges,
    };

    todo!()
}

pub fn compute_prev_challenges(
    old_bulletproof_challenges: [[BulletproofChallenge; WRAP_SCALARS_PER_CHALLENGE];
        WRAP_PREV_CHALLENGES],
    challenge_polynomial_commitments: [HexPointCoordinates; WRAP_PREV_CHALLENGES],
) -> Result<Vec<RecursionChallenge<Pallas>>, String> {
    let mut recursion_challenges = Vec::with_capacity(WRAP_PREV_CHALLENGES);

    for (chal, comm) in old_bulletproof_challenges
        .into_iter()
        .zip(challenge_polynomial_commitments.into_iter())
    {
        let mut chals = Vec::with_capacity(WRAP_SCALARS_PER_CHALLENGE);
        for prechallenge in chal.into_iter().map(|chal| chal.prechallenge) {
            let [limb0, limb1] = prechallenge.inner;

            let limb0 = u64::from_be_bytes(
                limb0
                    .parse::<i64>()
                    .map_err(|err| err.to_string())?
                    .to_be_bytes(),
            ) as u128;
            let limb1 = u64::from_be_bytes(
                limb1
                    .parse::<i64>()
                    .map_err(|err| err.to_string())?
                    .to_be_bytes(),
            ) as u128;

            let field = Fq::from(limb0 | (limb1 << 64));

            let (_, endo_r) = endos::<Pallas>();
            chals.push(ScalarChallenge(field).to_field(&endo_r));
        }

        let comm = PolyComm {
            elems: vec![WrapECPoint::try_from(comm)?.0],
        };

        recursion_challenges.push(RecursionChallenge { chals, comm });
    }

    Ok(recursion_challenges)
}

#[cfg(test)]
mod tests {
    use kimchi::{
        mina_curves::pasta::{Fp, Fq, Pallas},
        poly_commitment::PolyComm,
    };
    use o1_utils::FieldHelpers;

    use crate::pickles_preproc::state_proof::{
        BulletproofChallenge, Prechallenge, WRAP_SCALARS_PER_CHALLENGE,
    };

    use super::compute_prev_challenges;

    #[test]
    fn prev_challenges_tests() {
        // reference values were taken from OpenMina's tests, and checked by calling Mina's
        // `to_field()` OCaml function.
        // https://github.com/openmina/openmina/blob/main/ledger/src/proofs/public_input/scalar_challenge.rs#L120

        let bulletproof_challenges_1: [BulletproofChallenge; WRAP_SCALARS_PER_CHALLENGE] = [
            ["7486980280913238963", "4173194488927267133"],
            ["-8437921285878338178", "-2241273202573544127"],
            ["7651331705457292674", "-3583141513394030281"],
            ["-3464302417307075879", "-436261906098457727"],
            ["8255044994932440761", "5640094314955753085"],
            ["-2513734760972484960", "1161566061253204655"],
            ["7525998242613288472", "3436443803216159028"],
            ["6809231383204761158", "-1877195934091894696"],
            ["-2746520749286704399", "-3783224604272248786"],
            ["-36686536733916892", "-7835584350097226223"],
            ["-487486487490201322", "2756145684490201109"],
            ["-2928903316653004982", "346819656816504982"],
            ["-6510054999844554738", "5242613218253829938"],
            ["-9192160905410203809", "9069127704639200224"],
            ["-1805085648820294365", "4705625510417283644"],
        ]
        .map(|prechallenge| BulletproofChallenge {
            prechallenge: Prechallenge {
                inner: prechallenge.map(str::to_string),
            },
        });
        let bulletproof_challenges_0: [BulletproofChallenge; WRAP_SCALARS_PER_CHALLENGE] = [
            ["7486980280913238963", "4173194488927267133"],
            ["-8437921285878338178", "-2241273202573544127"],
            ["7651331705457292674", "-3583141513394030281"],
            ["-3464302417307075879", "-436261906098457727"],
            ["8255044994932440761", "5640094314955753085"],
            ["-2513734760972484960", "1161566061253204655"],
            ["7525998242613288472", "3436443803216159028"],
            ["6809231383204761158", "-1877195934091894696"],
            ["-2746520749286704399", "-3783224604272248786"],
            ["-36686536733916892", "-7835584350097226223"],
            ["-487486487490201322", "2756145684490201109"],
            ["-2928903316653004982", "346819656816504982"],
            ["-6510054999844554738", "5242613218253829938"],
            ["-9192160905410203809", "9069127704639200224"],
            ["-1805085648820294365", "4705625510417283644"],
        ]
        .map(|prechallenge| BulletproofChallenge {
            prechallenge: Prechallenge {
                inner: prechallenge.map(str::to_string),
            },
        });

        let old_bulletproof_challenges = [bulletproof_challenges_0, bulletproof_challenges_1];
        let challenge_polynomial_commitments = [
            [
                "40000000000000000000000000000000224698FC094CF91B992D30ED00000000",
                "2",
            ]
            .map(str::to_string),
            [
                "40000000000000000000000000000000224698FC094CF91B992D30ED00000000",
                "2",
            ]
            .map(str::to_string),
        ];

        let prev_challenges =
            compute_prev_challenges(old_bulletproof_challenges, challenge_polynomial_commitments)
                .unwrap();

        let ocaml_results = [
            "A922DE10BD19D506BE5E5240287AFCD2F809D02FF05EDFA4AF352AEE2353DA29",
            "242A7448F2F61420595B4E9B15D7D14E1B804FDE3C12E3B0D22F980D4A60521D",
            "B412F4446AB1ABD794F3D9F07F37A70DA9EB34E70A61945F34A0F21B5D7D4007",
            "C57F9E4102427BF3EBB6B634815A7ED8DEAC7CD0E66BE4CE6FF3CC072156350D",
            "739B1283E3850AEF8F7B8206FE2FAE458692CAE72FC44B0F9C0E4059D04A1808",
            "03B3F202827A43CE0A5C6168C62EE07623F50FA0D15C75782462E9A59FBB503E",
            "35F8321ACA4EEFAB50E354E656A802D2D7981BF1F534D8B9A2C59482A4537906",
            "9F3C63C9296EEB7AFCC6EFCB347DAB907CC633FB79C75A919BE9036D89B6AD11",
            "02E73F47455B53F464FDBE744507A1767D12531FC731D0ADCBC1530630B8172F",
            "4F8D1EA54FD29176EBF6D8421769D3B4CEAFACAD47C4ABAAA702F99532C98F2D",
            "A44B7B1491C7396203C1A5D0BB7B1FC6DA0FB85FC5D7A2DE0710C4A7A4916B2D",
            "345ECA59C151C182D4AD1A0A10B29E181C30236E6EB64C688F5B5B0221A2BD28",
            "391609C56024448F2B05A61D9116AB3C56496C9E65A8BF019476C0B029ECD826",
            "C611063B76383AC746012DDDD9E2D831A00B15FE22BC211BF383A0391FAB4814",
            "730FC9A8BF1DCCE3E36286BEB51EAB8F0B0092A8FEE6901FAF158D1E5DBFBE05",
        ]
        .map(|hex| Fq::from_hex(hex).unwrap());

        assert_eq!(prev_challenges[0].chals, ocaml_results);
        assert_eq!(
            prev_challenges[0].comm,
            PolyComm {
                elems: vec![Pallas::new(Fp::from(-1), Fp::from(2), false)]
            }
        );
    }
}
