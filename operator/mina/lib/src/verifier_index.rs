use std::{array, sync::Arc};

use ark_ff::{Field, PrimeField};
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, Radix2EvaluationDomain, UVPolynomial,
};
use kimchi::{
    circuits::{
        constraints::FeatureFlags,
        expr::Linearization,
        lookup::lookups::{LookupFeatures, LookupPatterns},
    },
    linearization::expr_linearization,
    mina_curves::pasta::{Fp, Fq, Pallas, Vesta},
    poly_commitment::{srs::SRS, PolyComm},
    verifier_index::VerifierIndex,
};
use serde::Deserialize;

const DEVNET_VK_JSON: &str = include_str!("devnet_vk.json");
const MAINNET_VK_JSON: &str = include_str!("mainnet_vk.json");

pub enum MinaChain {
    Devnet,
    Mainnet,
}

#[derive(Deserialize)]
struct BlockchainVerificationKey {
    commitments: Commitments,
    index: Index,
}

#[derive(Deserialize)]
struct Commitments {
    sigma_comm: [JSONPolyComm; 7],
    coefficients_comm: [JSONPolyComm; 15],
    generic_comm: JSONPolyComm,
    psm_comm: JSONPolyComm,
    complete_add_comm: JSONPolyComm,
    mul_comm: JSONPolyComm,
    emul_comm: JSONPolyComm,
    endomul_scalar_comm: JSONPolyComm,
}

#[derive(Deserialize)]
struct Index {
    domain: Domain,
    max_poly_size: usize,
    public: usize,
    prev_challenges: usize,
    shifts: [JSONFq; 7],
}

#[derive(Deserialize)]
struct Domain {
    log_size_of_group: usize,
}

#[derive(Deserialize)]
struct JSONFp(String);
#[derive(Deserialize)]
struct JSONFq(String);

#[derive(Deserialize)]
struct JSONGroupAffine(JSONFp, JSONFp);

#[derive(Deserialize)]
struct JSONPolyComm(JSONGroupAffine);

impl TryInto<Fp> for JSONFp {
    type Error = String;

    fn try_into(self) -> Result<Fp, Self::Error> {
        let bytes = hex::decode(self.0.trim_start_matches("0x")).map_err(|err| err.to_string())?;
        Ok(Fp::from_be_bytes_mod_order(&bytes))
    }
}

impl TryInto<Fq> for JSONFq {
    type Error = String;

    fn try_into(self) -> Result<Fq, Self::Error> {
        let bytes = hex::decode(self.0.trim_start_matches("0x")).map_err(|err| err.to_string())?;
        Ok(Fq::from_be_bytes_mod_order(&bytes))
    }
}

impl TryInto<Pallas> for JSONGroupAffine {
    type Error = String;

    fn try_into(self) -> Result<Pallas, Self::Error> {
        // FIXME(xqft): handle point at infinity
        Ok(Pallas::new(self.0.try_into()?, self.1.try_into()?, false))
    }
}

impl TryInto<PolyComm<Pallas>> for JSONPolyComm {
    type Error = String;

    fn try_into(self) -> Result<PolyComm<Pallas>, Self::Error> {
        Ok(PolyComm {
            unshifted: vec![self.0.try_into()?],
            shifted: None,
        })
    }
}

pub fn deserialize_blockchain_vk(chain: MinaChain) -> Result<VerifierIndex<Pallas>, String> {
    let vk_json = match chain {
        MinaChain::Devnet => DEVNET_VK_JSON,
        MinaChain::Mainnet => MAINNET_VK_JSON,
    };
    let vk: BlockchainVerificationKey =
        serde_json::from_str(vk_json).map_err(|err| err.to_string())?;

    let max_poly_size = vk.index.max_poly_size;
    let domain = Radix2EvaluationDomain::new(1 << vk.index.domain.log_size_of_group)
        .ok_or("failed to create domain".to_owned())?;

    let Commitments {
        sigma_comm,
        coefficients_comm,
        generic_comm,
        psm_comm,
        complete_add_comm,
        mul_comm,
        emul_comm,
        endomul_scalar_comm,
    } = vk.commitments;
    let empty_poly_comm = |_| PolyComm::new(Vec::new(), None);

    let sigma_comm = {
        let mut new_sigma_comm = array::from_fn(empty_poly_comm);
        for (comm, new_comm) in sigma_comm.into_iter().zip(new_sigma_comm.iter_mut()) {
            *new_comm = comm.try_into()?
        }
        new_sigma_comm
    };
    let coefficients_comm = {
        let mut new_coefficients_comm = array::from_fn(empty_poly_comm);
        for (comm, new_comm) in coefficients_comm
            .into_iter()
            .zip(new_coefficients_comm.iter_mut())
        {
            *new_comm = comm.try_into()?
        }
        new_coefficients_comm
    };
    let generic_comm = generic_comm.try_into()?;
    let psm_comm = psm_comm.try_into()?;
    let complete_add_comm = complete_add_comm.try_into()?;
    let mul_comm = mul_comm.try_into()?;
    let emul_comm = emul_comm.try_into()?;
    let endomul_scalar_comm = endomul_scalar_comm.try_into()?;

    let shift = vk.index.shifts;
    let shift = {
        let mut new_shift = array::from_fn(|_| Fq::from(0));
        for (comm, new_comm) in shift.into_iter().zip(new_shift.iter_mut()) {
            *new_comm = comm.try_into()?
        }
        new_shift
    };

    // The code below was taken from OpenMina
    // https://github.com/openmina/openmina/blob/main/ledger/src/proofs/verifier_index.rs#L151

    let (endo, _) = poly_commitment::srs::endos::<Vesta>();

    let feature_flags = FeatureFlags {
        range_check0: false,
        range_check1: false,
        foreign_field_add: false,
        foreign_field_mul: false,
        xor: false,
        rot: false,
        lookup_features: LookupFeatures {
            patterns: LookupPatterns {
                xor: false,
                lookup: false,
                range_check: false,
                foreign_field_mul: false,
            },
            joint_lookup_used: false,
            uses_runtime_tables: false,
        },
    };

    let (mut linearization, powers_of_alpha) = expr_linearization(Some(&feature_flags), true);

    let linearization = Linearization {
        constant_term: linearization.constant_term,
        index_terms: {
            // Make the verifier index deterministic
            linearization
                .index_terms
                .sort_by_key(|&(columns, _)| columns);
            linearization.index_terms
        },
    };

    // https://github.com/o1-labs/proof-systems/blob/2702b09063c7a48131173d78b6cf9408674fd67e/kimchi/src/verifier_index.rs#L310-L314
    let srs = {
        let mut srs = SRS::create(max_poly_size);
        srs.add_lagrange_basis(domain);
        Arc::new(srs)
    };

    // https://github.com/o1-labs/proof-systems/blob/2702b09063c7a48131173d78b6cf9408674fd67e/kimchi/src/verifier_index.rs#L319
    let zkpm = zk_polynomial(domain);

    // https://github.com/o1-labs/proof-systems/blob/2702b09063c7a48131173d78b6cf9408674fd67e/kimchi/src/verifier_index.rs#L324
    let w = zk_w3(domain);

    Ok(VerifierIndex {
        domain,
        max_poly_size: vk.index.max_poly_size,
        srs: once_cell::sync::OnceCell::from(srs),
        public: vk.index.public,
        prev_challenges: vk.index.prev_challenges,

        sigma_comm,
        coefficients_comm,
        generic_comm,
        psm_comm,
        complete_add_comm,
        mul_comm,
        emul_comm,
        endomul_scalar_comm,

        range_check0_comm: None,
        range_check1_comm: None,
        foreign_field_add_comm: None,
        foreign_field_mul_comm: None,
        xor_comm: None,
        rot_comm: None,

        shift,
        zkpm: once_cell::sync::OnceCell::from(zkpm),
        w: once_cell::sync::OnceCell::from(w),
        endo,
        lookup_index: None,
        linearization,
        powers_of_alpha,
    })
}

// The code below was taken from OpenMina
// https://github.com/openmina/openmina/blob/main/ledger/src/proofs/verifier_index.rs#L151
/// Returns the end of the circuit, which is used for introducing zero-knowledge in the permutation polynomial
pub fn zk_w3(domain: Radix2EvaluationDomain<Fq>) -> Fq {
    const ZK_ROWS: u64 = 3;
    domain.group_gen.pow([domain.size - (ZK_ROWS)])
}

// The code below was taken from OpenMina
// https://github.com/openmina/openmina/blob/main/ledger/src/proofs/verifier_index.rs#L151
/// Computes the zero-knowledge polynomial for blinding the permutation polynomial: `(x-w^{n-k})(x-w^{n-k-1})...(x-w^n)`.
/// Currently, we use k = 3 for 2 blinding factors,
/// see <https://www.plonk.cafe/t/noob-questions-plonk-paper/73>
pub fn zk_polynomial(domain: Radix2EvaluationDomain<Fq>) -> DensePolynomial<Fq> {
    let w3 = zk_w3(domain);
    let w2 = domain.group_gen * w3;
    let w1 = domain.group_gen * w2;

    // (x-w3)(x-w2)(x-w1) =
    // x^3 - x^2(w1+w2+w3) + x(w1w2+w1w3+w2w3) - w1w2w3
    let w1w2 = w1 * w2;
    DensePolynomial::from_coefficients_slice(&[
        -w1w2 * w3,                   // 1
        w1w2 + (w1 * w3) + (w3 * w2), // x
        -w1 - w2 - w3,                // x^2
        Fq::from(1),                  // x^3
    ])
}

#[cfg(test)]
mod test {
    use super::{deserialize_blockchain_vk, MinaChain};

    #[test]
    fn deserialize_blockchain_vk_does_not_fail() {
        deserialize_blockchain_vk(MinaChain::Devnet).unwrap();
    }
}
