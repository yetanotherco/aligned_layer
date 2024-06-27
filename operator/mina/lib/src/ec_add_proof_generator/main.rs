use std::array;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Arc;

use ark_ec::short_weierstrass_jacobian::GroupAffine;
use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::Field;
use ark_ff::One;
use ark_ff::PrimeField;
use ark_ff::UniformRand;
use ark_ff::Zero;
use ark_serialize::Write;
use kimchi::curve::KimchiCurve;
use kimchi::groupmap::GroupMap;
use kimchi::mina_curves::pasta::VestaParameters;
use kimchi::mina_poseidon::constants::PlonkSpongeConstantsKimchi;
use kimchi::mina_poseidon::sponge::DefaultFqSponge;
use kimchi::poly_commitment::commitment::CommitmentCurve;
use kimchi::poly_commitment::evaluation_proof::OpeningProof;
use kimchi::poly_commitment::srs::SRS;
use kimchi::proof::ProverProof;
use kimchi::prover_index::testing::new_index_for_test;
use kimchi::verifier::verify;
use kimchi::verifier_index::VerifierIndex;
use kimchi::{
    circuits::{
        gate::{CircuitGate, GateType},
        wires::{Wire, COLUMNS},
    },
    mina_curves::pasta::{Fp, Pallas, Vesta},
    mina_poseidon::sponge::DefaultFrSponge,
};
use serde::ser::Serialize;
use serde::Deserialize;

type SpongeParams = PlonkSpongeConstantsKimchi;
type BaseSponge = DefaultFqSponge<VestaParameters, SpongeParams>;
type ScalarSponge = DefaultFrSponge<Fp, SpongeParams>;

fn main() {
    use o1_utils::tests::make_test_rng;
    let mut rng = make_test_rng();

    let num_doubles = 5;
    let num_additions = 5;
    let num_infs = 5;

    let mut gates: Vec<CircuitGate<Fp>> = Vec::new();

    for row in 0..(num_doubles + num_additions + num_infs) {
        gates.push(CircuitGate::new(
            GateType::CompleteAdd,
            Wire::for_row(row),
            vec![],
        ));
    }

    let mut witness: [Vec<Fp>; COLUMNS] = array::from_fn(|_| vec![]);

    let ps = {
        let p = Pallas::prime_subgroup_generator()
            .into_projective()
            .mul(<Pallas as AffineCurve>::ScalarField::rand(&mut rng).into_repr())
            .into_affine();
        let mut res = vec![];
        let mut acc = p;
        for _ in 0..num_additions {
            res.push(acc);
            acc = acc + p;
        }
        res
    };

    let qs = {
        let q = Pallas::prime_subgroup_generator()
            .into_projective()
            .mul(<Pallas as AffineCurve>::ScalarField::rand(&mut rng).into_repr())
            .into_affine();
        let mut res = vec![];
        let mut acc = q;
        for _ in 0..num_additions {
            res.push(acc);
            acc = acc + q;
        }
        res
    };

    for &p in ps.iter().take(num_doubles) {
        let p2: Pallas = p + p;
        let (x1, y1) = (p.x, p.y);
        let x1_squared = x1.square();
        // 2 * s * y1 = 3 * x1^2
        let s = (x1_squared.double() + x1_squared) / y1.double();

        witness[0].push(p.x);
        witness[1].push(p.y);
        witness[2].push(p.x);
        witness[3].push(p.y);
        witness[4].push(p2.x);
        witness[5].push(p2.y);
        witness[6].push(Fp::zero());
        witness[7].push(Fp::one());
        witness[8].push(s);
        witness[9].push(Fp::zero());
        witness[10].push(Fp::zero());

        witness[11].push(Fp::zero());
        witness[12].push(Fp::zero());
        witness[13].push(Fp::zero());
        witness[14].push(Fp::zero());
    }

    for i in 0..num_additions {
        let p = ps[i];
        let q = qs[i];

        let pq: Pallas = p + q;
        let (x1, y1) = (p.x, p.y);
        let (x2, y2) = (q.x, q.y);
        // (x2 - x1) * s = y2 - y1
        let s = (y2 - y1) / (x2 - x1);
        witness[0].push(x1);
        witness[1].push(y1);
        witness[2].push(x2);
        witness[3].push(y2);
        witness[4].push(pq.x);
        witness[5].push(pq.y);
        witness[6].push(Fp::zero());
        witness[7].push(Fp::zero());
        witness[8].push(s);
        witness[9].push(Fp::zero());
        witness[10].push((x2 - x1).inverse().unwrap());

        witness[11].push(Fp::zero());
        witness[12].push(Fp::zero());
        witness[13].push(Fp::zero());
        witness[14].push(Fp::zero());
    }

    for &p in ps.iter().take(num_infs) {
        let q: Pallas = -p;

        let p2: Pallas = p + p;
        let (x1, y1) = (p.x, p.y);
        let x1_squared = x1.square();
        // 2 * s * y1 = -3 * x1^2
        let s = (x1_squared.double() + x1_squared) / y1.double();
        witness[0].push(p.x);
        witness[1].push(p.y);
        witness[2].push(q.x);
        witness[3].push(q.y);
        witness[4].push(p2.x);
        witness[5].push(p2.y);
        witness[6].push(Fp::one());
        witness[7].push(Fp::one());
        witness[8].push(s);
        witness[9].push((q.y - p.y).inverse().unwrap());
        witness[10].push(Fp::zero());

        witness[11].push(Fp::zero());
        witness[12].push(Fp::zero());
        witness[13].push(Fp::zero());
        witness[14].push(Fp::zero());
    }

    let prover_index = new_index_for_test::<Vesta>(gates, 0);
    let group_map = <Vesta as CommitmentCurve>::Map::setup();

    let proof = ProverProof::create_recursive::<BaseSponge, ScalarSponge>(
        &group_map,
        witness,
        &Vec::new(),
        &prover_index,
        Vec::new(),
        None,
    )
    .unwrap();

    let verifier_index = prover_index.verifier_index();

    let verification_result = verify::<
        GroupAffine<VestaParameters>,
        BaseSponge,
        ScalarSponge,
        OpeningProof<Vesta>,
    >(&group_map, &verifier_index, &proof, &Vec::new())
    .is_ok();

    println!("VERIFICATION RESULT: {}", verification_result);
    if !verification_result {
        panic!("Proof did not verify")
    }

    let verifier_index_file_path = Path::new("kimchi_verifier_index.bin");
    std::fs::File::create(verifier_index_file_path).unwrap();
    verifier_index
        .to_file(verifier_index_file_path, None)
        .expect("Could not write verifier index");

    let verifier_index_file = std::fs::File::create(verifier_index_file_path)
        .expect("Could not create verifier index file");
    let verifier_index_writer = BufWriter::new(verifier_index_file);
    verifier_index
        .serialize(&mut rmp_serde::Serializer::new(verifier_index_writer))
        .expect("Could not serialize verifier index");

    let proof_file_path = Path::new("kimchi_ec_add.proof");
    let proof_file = std::fs::File::create(proof_file_path).unwrap();
    let writer = std::io::BufWriter::new(proof_file);
    proof
        .serialize(&mut rmp_serde::Serializer::new(writer))
        .expect("Could not serialize kimchi proof");

    println!(
        "Kimchi verifier index written into {:?}",
        verifier_index_file_path
    );
    println!("Kimchi proof written into {:?}", proof_file_path);

    // # ----------- PURE VERIFIER SECTION ---------------- #

    let deserialized_proof_file =
        std::fs::File::open(proof_file_path).expect("Could not open kimchi proof file");
    let proof_reader = BufReader::new(deserialized_proof_file);
    let deserialized_proof: ProverProof<Vesta, OpeningProof<Vesta>> =
        ProverProof::deserialize(&mut rmp_serde::Deserializer::new(proof_reader))
            .expect("Could not deserialize kimchi proof");

    let deserialized_verifier_index_file =
        std::fs::File::open(verifier_index_file_path).expect("Could not open verifier index file");
    let deserialized_verifier_index_reader = BufReader::new(deserialized_verifier_index_file);
    let mut deserialized_verifier_index: VerifierIndex<Vesta, OpeningProof<Vesta>> =
        VerifierIndex::deserialize(&mut rmp_serde::Deserializer::new(
            deserialized_verifier_index_reader,
        ))
        .expect("Could not deserialize verifier index");

    let endo = *Vesta::other_curve_endo();
    let mut srs = SRS::<Vesta>::create(verifier_index.max_poly_size);
    srs.add_lagrange_basis(deserialized_verifier_index.domain);

    deserialized_verifier_index.endo = endo;
    deserialized_verifier_index.srs = Arc::new(srs.clone());

    verify::<GroupAffine<VestaParameters>, BaseSponge, ScalarSponge, OpeningProof<Vesta>>(
        &group_map,
        &deserialized_verifier_index,
        &deserialized_proof,
        &Vec::new(),
    )
    .expect("Deserialized kimchi proof did not verify");

    println!("Deserialized proof verified successfully");

    let verifier_index_bytes = rmp_serde::to_vec(&deserialized_verifier_index)
        .expect("Could not serialize verifier index");
    let verifier_index_file_path = Path::new("kimchi_verifier_index.bin");
    let verifier_index_file = std::fs::File::create(verifier_index_file_path)
        .expect("Could not create kimchi aggregated pub input file");
    let mut verifier_index_writer = BufWriter::new(verifier_index_file);
    verifier_index_writer
        .write_all(&verifier_index_bytes)
        .expect("Could not serialize kimchi aggregated pub input");

    println!(
        "Kimchi verifier index (public input) written into {:?}",
        verifier_index_file_path
    );
}
