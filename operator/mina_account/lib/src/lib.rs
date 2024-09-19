use alloy::sol_types::SolValue;
use merkle_verifier::verify_merkle_proof;
use mina_bridge_core::{
    proof::account_proof::{MinaAccountProof, MinaAccountPubInputs},
    sol::account::MinaAccountValidation,
};
use mina_tree::Account;

mod merkle_verifier;

// TODO(xqft): check sizes
const MAX_PROOF_SIZE: usize = 16 * 1024;
const MAX_PUB_INPUT_SIZE: usize = 6 * 1024;

#[no_mangle]
pub extern "C" fn verify_account_inclusion_ffi(
    proof_buffer: &[u8; MAX_PROOF_SIZE],
    proof_len: usize,
    pub_input_buffer: &[u8; MAX_PUB_INPUT_SIZE],
    pub_input_len: usize,
) -> bool {
    let Some(proof_buffer_slice) = proof_buffer.get(..proof_len) else {
        eprintln!("Proof length argument is greater than max proof size");
        return false;
    };

    let Some(pub_input_buffer_slice) = pub_input_buffer.get(..pub_input_len) else {
        eprintln!("Public input length argument is greater than max public input size");
        return false;
    };

    let MinaAccountProof {
        merkle_path,
        account,
    } = match bincode::deserialize(proof_buffer_slice) {
        Ok(proof) => proof,
        Err(err) => {
            eprintln!("Failed to deserialize account proof: {}", err);
            return false;
        }
    };
    let MinaAccountPubInputs {
        ledger_hash,
        encoded_account,
    } = match bincode::deserialize(pub_input_buffer_slice) {
        Ok(pub_inputs) => pub_inputs,
        Err(err) => {
            eprintln!("Failed to deserialize account pub inputs: {}", err);
            return false;
        }
    };

    let expected_encoded_account = match MinaAccountValidation::Account::try_from(&account) {
        Ok(account) => account,
        Err(err) => {
            eprintln!("Failed to convert Mina account to Solidity struct: {}", err);
            return false;
        }
    }
    .abi_encode();
    if expected_encoded_account != encoded_account {
        eprintln!("ABI encoded account in public inputs doesn't match the account on the proof");
        return false;
    }

    // the hash function for MinaBaseAccountBinableArgStableV2 produces a panic every
    // time it's called. So we use Account's one.
    let leaf_hash = Account::from(&account).hash();

    // TODO(xqft): when the needed account GraphQL query is done, do:
    // 1. send encoded account as part of the proof
    // 2. check poseidon(account) == leaf_hash
    // 3. check keccak(account) == account_hash

    verify_merkle_proof(leaf_hash, merkle_path, ledger_hash)
}

#[cfg(test)]
mod test {

    use super::*;

    const PROOF_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina_account/mina_account.proof");
    const PUB_INPUT_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina_account/mina_account.pub");

    #[test]
    fn valid_account_state_proof_verifies() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PUB_INPUT_BYTES);

        let result = verify_account_inclusion_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(result);
    }

    #[test]
    fn empty_account_state_proof_does_not_verify() {
        let proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PUB_INPUT_BYTES);

        let result = verify_account_inclusion_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }

    #[test]
    fn valid_account_state_proof_with_empty_pub_input_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();

        let result = verify_account_inclusion_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }

    #[test]
    fn valid_account_state_proof_with_greater_proof_size_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let wrong_proof_size = MAX_PROOF_SIZE + 1;
        proof_buffer[..PROOF_BYTES.len()].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let pub_input_size = PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PUB_INPUT_BYTES);

        let result = verify_account_inclusion_ffi(
            &proof_buffer,
            wrong_proof_size,
            &pub_input_buffer,
            pub_input_size,
        );
        assert!(!result);
    }

    #[test]
    fn valid_account_state_proof_with_greater_pub_input_size_does_not_verify() {
        let mut proof_buffer = [0u8; super::MAX_PROOF_SIZE];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; super::MAX_PUB_INPUT_SIZE];
        let wrong_pub_input_size = MAX_PUB_INPUT_SIZE + 1;
        pub_input_buffer[..PUB_INPUT_BYTES.len()].clone_from_slice(PUB_INPUT_BYTES);

        let result = verify_account_inclusion_ffi(
            &proof_buffer,
            proof_size,
            &pub_input_buffer,
            wrong_pub_input_size,
        );
        assert!(!result);
    }
}
