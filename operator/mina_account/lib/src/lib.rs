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
    let MinaAccountProof {
        merkle_path,
        account,
    } = match bincode::deserialize(&proof_buffer[..proof_len]) {
        Ok(proof) => proof,
        Err(err) => {
            eprintln!("Failed to deserialize account proof: {}", err);
            return false;
        }
    };
    let MinaAccountPubInputs {
        ledger_hash,
        encoded_account,
    } = match bincode::deserialize(&pub_input_buffer[..pub_input_len]) {
        Ok(pub_inputs) => pub_inputs,
        Err(err) => {
            eprintln!("Failed to deserialize account pub inputs: {}", err);
            return false;
        }
    };

    let expected_encoded_account = MinaAccountValidation::Account::try_from(&account)?.abi_encode();
    if expected_encoded_account != encoded_account {
        return Err("ABI encoded account in public inputs doesn't match the account on the proof");
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
