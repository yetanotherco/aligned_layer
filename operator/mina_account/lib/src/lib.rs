use alloy::sol_types::SolValue;
use log::error;
use merkle_verifier::verify_merkle_proof;
use mina_bridge_core::{
    proof::account_proof::{MinaAccountProof, MinaAccountPubInputs},
    sol::account::MinaAccountValidationExample,
};
use mina_tree::Account;

mod merkle_verifier;

#[no_mangle]
pub extern "C" fn verify_account_inclusion_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    pub_input_bytes: *const u8,
    pub_input_len: u32,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        inner_verify_account_inclusion_ffi(proof_bytes, proof_len, pub_input_bytes, pub_input_len)
    });

    match result {
        Ok(v) => v as i32,
        Err(_) => -1,
    }
}

fn inner_verify_account_inclusion_ffi(
    proof_bytes: *const u8,
    proof_len: u32,
    pub_input_bytes: *const u8,
    pub_input_len: u32,
) -> bool {
    if proof_bytes.is_null() || pub_input_bytes.is_null() {
        error!("Input buffer null");
        return false;
    }

    if proof_len == 0 || pub_input_len == 0 {
        error!("Input buffer length zero size");
        return false;
    }

    let proof_bytes = unsafe { std::slice::from_raw_parts(proof_bytes, proof_len as usize) };

    let pub_input_bytes =
        unsafe { std::slice::from_raw_parts(pub_input_bytes, pub_input_len as usize) };

    verify_account_inclusion(proof_bytes, pub_input_bytes)
}

/// Verifies a Mina account inclusion.
/// Verifies that exists a Merkle path composed of:
///
/// - The hash of the Mina account as the leaf
/// - The ledger hash as the root
/// - Some intermediate nodes
///
/// Returns `true` if the Mina account is included in the ledger hash. `false` otherwise.
pub fn verify_account_inclusion(proof_bytes: &[u8], pub_input_bytes: &[u8]) -> bool {
    let MinaAccountProof {
        merkle_path,
        account,
    } = match bincode::deserialize(proof_bytes) {
        Ok(proof) => proof,
        Err(err) => {
            error!("Failed to deserialize account proof: {}", err);
            return false;
        }
    };
    let MinaAccountPubInputs {
        ledger_hash,
        encoded_account,
    } = match bincode::deserialize(pub_input_bytes) {
        Ok(pub_inputs) => pub_inputs,
        Err(err) => {
            error!("Failed to deserialize account pub inputs: {}", err);
            return false;
        }
    };

    let expected_encoded_account =
        match MinaAccountValidationExample::Account::try_from(&account) {
            Ok(account) => account,
            Err(err) => {
                error!("Failed to convert Mina account to Solidity struct: {}", err);
                return false;
            }
        }
        .abi_encode();
    if expected_encoded_account != encoded_account {
        error!("ABI encoded account in public inputs doesn't match the account on the proof");
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
    use core::ptr;

    const PROOF_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina_account/mina_account.proof");
    const PUB_INPUT_BYTES: &[u8] =
        include_bytes!("../../../../scripts/test_files/mina_account/mina_account.pub");

    #[test]
    fn valid_account_state_proof_verifies() {
        let mut proof_buffer = [0u8; PROOF_BYTES.len()];
        let proof_size = PROOF_BYTES.len();
        assert!(proof_size <= proof_buffer.len());
        proof_buffer[..proof_size].clone_from_slice(PROOF_BYTES);

        let mut pub_input_buffer = [0u8; PUB_INPUT_BYTES.len()];
        let pub_input_size = PUB_INPUT_BYTES.len();
        assert!(pub_input_size <= pub_input_buffer.len());
        pub_input_buffer[..pub_input_size].clone_from_slice(PUB_INPUT_BYTES);

        let result = verify_account_inclusion_ffi(
            proof_buffer.as_ptr(),
            proof_size as u32,
            pub_input_buffer.as_ptr(),
            pub_input_size as u32,
        );
        assert_eq!(result, 1);
    }

    #[test]
    fn zeroized_account_state_proof_does_not_verify() {
        const PROOF_SIZE: usize = PROOF_BYTES.len();
        let proof_buffer = [0u8; PROOF_SIZE];

        let result = verify_account_inclusion_ffi(
            proof_buffer.as_ptr(),
            PROOF_SIZE as u32,
            PUB_INPUT_BYTES.as_ptr(),
            PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn valid_account_state_proof_with_zeroized_pub_input_does_not_verify() {
        const PUB_INPUT_SIZE: usize = PUB_INPUT_BYTES.len();
        let pub_input_buffer = [0u8; PUB_INPUT_SIZE];

        let result = verify_account_inclusion_ffi(
            PROOF_BYTES.as_ptr(),
            PROOF_BYTES.len() as u32,
            pub_input_buffer.as_ptr(),
            PUB_INPUT_SIZE as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn null_account_state_proof_does_not_verify() {
        let result = verify_account_inclusion_ffi(
            ptr::null(),
            PROOF_BYTES.len() as u32,
            PUB_INPUT_BYTES.as_ptr(),
            PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn valid_account_state_proof_with_null_pub_input_does_not_verify() {
        let result = verify_account_inclusion_ffi(
            PROOF_BYTES.as_ptr(),
            PROOF_BYTES.len() as u32,
            ptr::null(),
            PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn empty_account_state_proof_does_not_verify() {
        let result = verify_account_inclusion_ffi(
            PROOF_BYTES.as_ptr(),
            0,
            PUB_INPUT_BYTES.as_ptr(),
            PUB_INPUT_BYTES.len() as u32,
        );
        assert_eq!(result, 0);
    }

    #[test]
    fn valid_account_state_proof_with_empty_pub_input_does_not_verify() {
        let result = verify_account_inclusion_ffi(
            PROOF_BYTES.as_ptr(),
            PROOF_BYTES.len() as u32,
            PUB_INPUT_BYTES.as_ptr(),
            0,
        );
        assert_eq!(result, 0);
    }
}
