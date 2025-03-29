use std::time::Duration;

use proof_aggregator::{aggregate_proofs, InputProofs, ProgramInput};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder().finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // simulate a service that aggregates proofs every n seconds after some processing
    loop {
        info!("Waiting 2 seconds before aggregating proofs...");
        std::thread::sleep(Duration::from_secs(2));

        let proofs = InputProofs::SP1Compressed(vec![]);
        let input = ProgramInput::new(proofs);
        let Ok(output) = aggregate_proofs(input) else {
            error!("Error while aggregating and verifying proofs");
            return;
        };

        info!("Proof aggregated, sending to aligned verification contract...");

        // TODO: send a blob transaction to with the merkle leaves
        // TODO: call contract to verify proof + attach blob transaction root
        // the contract should emit a log with the verification and the path to the blob
        let _calldata = output.calldata();
    }
}
