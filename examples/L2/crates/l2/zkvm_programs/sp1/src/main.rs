#![no_main]
sp1_zkvm::entrypoint!(main);

use lambdaworks_crypto::merkle_tree::merkle::MerkleTree;
use primitive_types::U256;
use sp1_state_transition_program::{ProgramInput, ProgramOutput};
use types::UserState;

pub fn main() {
    let mut input = sp1_zkvm::io::read::<ProgramInput>();

    let initial_state: Vec<UserState> = input.user_states.clone().into_values().collect();
    let initial_state_merkle_tree: MerkleTree<UserState> =
        MerkleTree::build(&initial_state).expect("to build merkle tree with the provided state");
    let initial_state_merkle_root = initial_state_merkle_tree.root;

    for transfer in input.transfers {
        let mut user_from = input
            .user_states
            .get(&transfer.from)
            .expect("User must exist in state")
            .clone();
        let mut user_to = input
            .user_states
            .get(&transfer.to)
            .expect("User must exist in state")
            .clone();

        if user_from.balance >= transfer.amount {
            user_from.balance -= transfer.amount;
            user_from.nonce += U256::one();
            user_to.balance += transfer.amount;
        } else {
            panic!("User does not have enough balance to perform the transfer",);
        }

        input.user_states.insert(transfer.from, user_from);
        input.user_states.insert(transfer.to, user_to);
    }

    let post_state: Vec<UserState> = input.user_states.clone().into_values().collect();
    let post_state_merkle_tree: MerkleTree<UserState> =
        MerkleTree::build(&post_state).expect("to build merkle tree with the provided state");
    let post_state_merkle_root = post_state_merkle_tree.root;

    let program_output = ProgramOutput {
        initial_state_merkle_root,
        post_state_merkle_root,
    };

    sp1_zkvm::io::commit(&program_output);
}
