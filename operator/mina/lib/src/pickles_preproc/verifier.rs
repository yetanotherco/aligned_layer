// use kimchi::{
//     groupmap::GroupMap,
//     mina_curves::pasta::{Fp, Fq, Pallas, PallasParameters},
//     mina_poseidon::{
//         constants::PlonkSpongeConstantsKimchi,
//         sponge::{DefaultFqSponge, DefaultFrSponge},
//     },
//     verifier::batch_verify,
// };

// pub fn verify() -> Result<(), String> {
//     let group_map = GroupMap::<Fp>::setup();

//     batch_verify::<
//         Pallas,
//         DefaultFqSponge<PallasParameters, PlonkSpongeConstantsKimchi>,
//         DefaultFrSponge<Fq, PlonkSpongeConstantsKimchi>,
//     >(group_map, proofs)
//     .map_err(|err| format!("Could not verify Kimchi proof: {err}"))
// }
