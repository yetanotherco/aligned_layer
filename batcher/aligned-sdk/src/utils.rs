use crate::errors::SubmitError;
use crate::models::ProvingSystemId;

pub fn parse_proving_system(proving_system: &str) -> Result<Option<ProvingSystemId>, SubmitError> {
    match proving_system {
        "GnarkPlonkBls12_381" => Ok(Some(ProvingSystemId::GnarkPlonkBls12_381)),
        "GnarkPlonkBn254" => Ok(Some(ProvingSystemId::GnarkPlonkBn254)),
        "Groth16Bn254" => Ok(Some(ProvingSystemId::Groth16Bn254)),
        "SP1" => Ok(Some(ProvingSystemId::SP1)),
        "Halo2IPA" => Ok(Some(ProvingSystemId::Halo2IPA)),
        "Halo2KZG" => Ok(Some(ProvingSystemId::Halo2KZG)),
        _ => Err(SubmitError::InvalidProvingSystem(
            proving_system.to_string(),
        )),
    }
}
