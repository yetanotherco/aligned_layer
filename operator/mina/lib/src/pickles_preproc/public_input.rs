use kimchi::mina_curves::pasta::{Fp, Fq};

use super::state_proof::Statement;

pub fn tock_unpadded_public_input_of_statement(prev_statement: Statement) -> Vec<Fq> {
    let prev_statement_as_fields = vec![];

    let fp = [prev_statement.proof_state.deferred_values];

    prev_statement_as_fields
}

fn deferred_values(statement: Statement) {}
