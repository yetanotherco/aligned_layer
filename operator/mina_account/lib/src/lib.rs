use merkle_verifier::verify_merkle_proof;
use mina_bridge_core::proof::account_proof::{MinaAccountProof, MinaAccountPubInputs};
use mina_p2p_messages::hash::MinaHash;

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
    let MinaAccountPubInputs { ledger_hash, .. } =
        match bincode::deserialize(&pub_input_buffer[..pub_input_len]) {
            Ok(pub_inputs) => pub_inputs,
            Err(err) => {
                eprintln!("Failed to deserialize account pub inputs: {}", err);
                return false;
            }
        };

    let leaf_hash = account.hash();

    // TODO(xqft): when the needed account GraphQL query is done, do:
    // 1. send encoded account as part of the proof
    // 2. check poseidon(account) == leaf_hash
    // 3. check keccak(account) == account_hash

    verify_merkle_proof(leaf_hash, merkle_path, ledger_hash)
}
