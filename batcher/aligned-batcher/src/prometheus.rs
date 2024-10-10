
// Prometheus
use prometheus::{
    IntCounter, IntGauge, Registry,
};

use warp::{Reply, Rejection};

use once_cell::sync::Lazy;

// Prometheus setup

pub static REGISTRY: Lazy<Registry> = Lazy::new(|| 
    Registry::new());

pub static OPEN_CONNECTIONS: Lazy<IntGauge> = Lazy::new(||
    IntGauge::new("open_connections", "Open Connections").expect("metric can be created"));

pub static RECEIVED_PROOFS: Lazy<IntCounter> = Lazy::new(||
    IntCounter::new("received_proofs", "Received Proofs").expect("metric can be created"));


// Register our custom metrics for prometheus
pub fn register_custom_metrics() {
    REGISTRY
        .register(Box::new(OPEN_CONNECTIONS.clone()))
        .expect("collector can be registered");

    REGISTRY
        .register(Box::new(RECEIVED_PROOFS.clone()))
        .expect("collector can be registered");
}

// so Prometheus can collect our metrics.
pub async fn metrics_handler() -> Result<impl Reply, Rejection> {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        eprintln!("could not encode custom metrics: {}", e);
    };
    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("custom metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        eprintln!("could not encode prometheus metrics: {}", e);
    };
    let res_custom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("prometheus metrics could not be from_utf8'd: {}", e);
            String::default()
        }
    };
    buffer.clear();

    res.push_str(&res_custom);
    Ok(res)
}
