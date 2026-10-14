use sp1_sdk::{LightProver, Prover, ProverClient, SP1ProofWithPublicValues, SP1VerifyingKey};
use tokio::sync::OnceCell;

/// Verify-only client, it does not generate proofs.
static SP1_VERIFIER_CLIENT: OnceCell<LightProver> = OnceCell::const_new();

pub enum VerificationError {
    InvalidProof,
    UnsupportedProof,
    UnsupportedVersion(String),
}

pub async fn verify_sp1_proof(
    proof: &SP1ProofWithPublicValues,
    vk: &SP1VerifyingKey,
) -> Result<(), VerificationError> {
    let client = SP1_VERIFIER_CLIENT
        .get_or_init(|| ProverClient::builder().light().build())
        .await;

    match proof.proof {
        sp1_sdk::SP1Proof::Compressed(_) => client.verify(proof, vk, None).map_err(|e| match e {
            sp1_sdk::SP1VerificationError::VersionMismatch(version) => {
                VerificationError::UnsupportedVersion(version)
            }
            _ => VerificationError::InvalidProof,
        }),
        _ => Err(VerificationError::UnsupportedProof),
    }?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../scripts/test_files/sp1");

    fn read_fixture(name: &str) -> Vec<u8> {
        std::fs::read(format!("{FIXTURES_DIR}/{name}")).expect("fixture to exist")
    }

    fn load_fibonacci_proof() -> (SP1ProofWithPublicValues, SP1VerifyingKey) {
        let proof = bincode::deserialize(&read_fixture("sp1_fibonacci_6_1_0.proof")).unwrap();
        let vk = bincode::deserialize(&read_fixture("sp1_fibonacci_6_1_0_vk.bin")).unwrap();
        (proof, vk)
    }

    #[tokio::test]
    async fn accepts_sp1_v6_compressed_proof() {
        let (proof, vk) = load_fibonacci_proof();

        assert!(verify_sp1_proof(&proof, &vk).await.is_ok());
    }

    #[tokio::test]
    async fn rejects_proof_with_tampered_public_values() {
        let (mut proof, vk) = load_fibonacci_proof();
        proof.public_values = sp1_sdk::SP1PublicValues::from(&[1, 2, 3]);

        let result = verify_sp1_proof(&proof, &vk).await;

        assert!(matches!(result, Err(VerificationError::InvalidProof)));
    }

    #[tokio::test]
    async fn rejects_proof_from_other_sp1_version() {
        let (mut proof, vk) = load_fibonacci_proof();
        proof.sp1_version = "v6.0.0".to_string();

        let result = verify_sp1_proof(&proof, &vk).await;

        assert!(matches!(
            result,
            Err(VerificationError::UnsupportedVersion(version)) if version == "v6.0.0"
        ));
    }

    // A v5 proof never decodes into a v6 proof. In release builds bincode returns an error,
    // while in debug builds a debug assertion of the KoalaBear field panics before that.
    #[test]
    fn sp1_v5_proof_is_not_deserializable() {
        let bytes = read_fixture("sp1_fibonacci_5_0_0.proof");

        let result =
            std::panic::catch_unwind(|| bincode::deserialize::<SP1ProofWithPublicValues>(&bytes));

        assert!(!matches!(result, Ok(Ok(_))));
    }
}
