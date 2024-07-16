use std::array;

use ark_poly::{EvaluationDomain, Radix2EvaluationDomain};
use kimchi::{
    mina_curves::pasta::{Fp, Fq, Pallas},
    o1_utils::FieldHelpers,
    poly_commitment::PolyComm,
    verifier_index::VerifierIndex,
};
use serde::Deserialize;

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
    evals: Evals,
    shifts: [JSONFq; 7],
}

#[derive(Deserialize)]
struct Domain {
    log_size_of_group: usize,
    group_gen: JSONFq,
}

#[derive(Deserialize)]
struct Evals {
    sigma_comm: [JSONPolyCommAlternative; 7],
    coefficients_comm: [JSONPolyCommAlternative; 15],
    generic_comm: JSONPolyCommAlternative,
    psm_comm: JSONPolyCommAlternative,
    complete_add_comm: JSONPolyCommAlternative,
    mul_comm: JSONPolyCommAlternative,
    emul_comm: JSONPolyCommAlternative,
    endomul_scalar_comm: JSONPolyCommAlternative,
}

#[derive(Deserialize)]
struct JSONFp(String);
#[derive(Deserialize)]
struct JSONFq(String);

#[derive(Deserialize)]
struct JSONGroupAffine(JSONFp, JSONFp);
#[derive(Deserialize)]
struct JSONGroupAffineAlternative(String, JSONGroupAffine);

#[derive(Deserialize)]
struct JSONPolyComm(JSONGroupAffine);
#[derive(Deserialize)]
struct JSONPolyCommAlternative {
    unshifted: [JSONGroupAffineAlternative; 1],
}

impl TryInto<Fp> for JSONFp {
    type Error = String;

    fn try_into(self) -> Result<Fp, Self::Error> {
        Fp::from_hex(&self.0).map_err(|err| err.to_string())
    }
}

impl TryInto<Fq> for JSONFq {
    type Error = String;

    fn try_into(self) -> Result<Fq, Self::Error> {
        Fq::from_hex(&self.0).map_err(|err| err.to_string())
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

pub fn deserialize_blockchain_vk(json_str: &str) -> Result<VerifierIndex<Pallas>, String> {
    let vk: BlockchainVerificationKey =
        serde_json::from_str(json_str).map_err(|err| err.to_string())?;
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
            new_comm = &mut comm.try_into()?
        }
        new_sigma_comm
    };
    let coefficients_comm = {
        let mut new_coefficients_comm = array::from_fn(empty_poly_comm);
        for (comm, new_comm) in coefficients_comm
            .into_iter()
            .zip(new_coefficients_comm.iter_mut())
        {
            new_comm = &mut comm.try_into()?
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
            new_comm = &mut comm.try_into()?
        }
        new_shift
    };

    let verifier_index = VerifierIndex {
        domain,
        max_poly_size: vk.index.max_poly_size,
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
    };
    todo!()
}

#[cfg(test)]
mod test {
    use super::deserialize_blockchain_vk;

    const BLOCKCHAIN_VK_JSON: &str =
        include_str!("../../../../batcher/aligned/test_files/mina/blockchain_vk.json");

    #[test]
    fn deserialize_blockchain_vk_does_not_fail() {
        deserialize_blockchain_vk(BLOCKCHAIN_VK_JSON).unwrap();
    }
}
