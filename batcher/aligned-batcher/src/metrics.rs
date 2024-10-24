// Prometheus
use prometheus::{opts, register_int_counter, register_int_gauge, IntCounter, IntGauge};

use warp::{Rejection, Reply};

use once_cell::sync::Lazy;
use std::{thread, time};

// Prometheus setup
pub static BATCHER_STARTED: Lazy<IntCounter> =
    Lazy::new(|| register_int_counter!(opts!("batcher_started", "Batcher Started")).unwrap());

pub static OPEN_CONNECTIONS: Lazy<IntGauge> =
    Lazy::new(|| register_int_gauge!(opts!("open_connections", "Open Connections")).unwrap());

pub static RECEIVED_PROOFS: Lazy<IntCounter> =
    Lazy::new(|| register_int_counter!(opts!("received_proofs", "Received Proofs")).unwrap());

pub static SENT_BATCHES: Lazy<IntCounter> =
    Lazy::new(|| register_int_counter!(opts!("sent_batches", "Sent Batches")).unwrap());

pub static REVERTED_BATCHES: Lazy<IntCounter> =
    Lazy::new(|| register_int_counter!(opts!("reverted_batches", "Reverted Batches")).unwrap());

pub static GAS_PRICE_USED_ON_LATEST_BATCH: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(opts!("gas_price_used_on_latest_batch", "Gas Price")).unwrap()
});

// so Prometheus can collect our metrics.
pub async fn metrics_handler() -> Result<impl Reply, Rejection> {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    };
    let res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    Ok(res)
}

pub fn init_variables() {
    BATCHER_STARTED.reset();

    OPEN_CONNECTIONS.set(0);

    RECEIVED_PROOFS.reset();

    SENT_BATCHES.reset();

    REVERTED_BATCHES.reset();

    GAS_PRICE_USED_ON_LATEST_BATCH.set(0);
}

pub fn batcher_started() {
    thread::sleep(time::Duration::from_secs(10));
    BATCHER_STARTED.inc();
}
