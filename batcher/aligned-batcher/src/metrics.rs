use std::{thread, time::Duration};

// Prometheus
use prometheus::{opts, register_int_counter, register_int_gauge, IntCounter, IntGauge};

use warp::{Filter, Rejection, Reply};

#[derive(Clone, Debug)]
pub struct BatcherMetrics {
    pub open_connections: IntGauge,
    pub received_proofs: IntCounter,
    pub sent_batches: IntCounter,
    pub reverted_batches: IntCounter,
    pub batcher_started: IntCounter,
    pub gas_price_used_on_latest_batch: IntGauge,
    pub broken_ws_connections: IntCounter,
}

impl BatcherMetrics {
    pub fn start(metrics_port: u16) -> anyhow::Result<Self> {
        let registry = prometheus::Registry::new();

        let open_connections = register_int_gauge!(opts!("open_connections", "Open Connections"))?;
        let received_proofs = register_int_counter!(opts!("received_proofs", "Received Proofs"))?;
        let sent_batches = register_int_counter!(opts!("sent_batches", "Sent Batches"))?;
        let reverted_batches =
            register_int_counter!(opts!("reverted_batches", "Reverted Batches"))?;
        let batcher_started = register_int_counter!(opts!("batcher_started", "Batcher Started"))?;
        let gas_price_used_on_latest_batch =
            register_int_gauge!(opts!("gas_price_used_on_latest_batch", "Gas Price"))?;
        let broken_ws_connections = register_int_counter!(opts!(
            "broken_ws_connections",
            "Broken websocket connections"
        ))?;

        registry.register(Box::new(open_connections.clone()))?;
        registry.register(Box::new(received_proofs.clone()))?;
        registry.register(Box::new(sent_batches.clone()))?;
        registry.register(Box::new(reverted_batches.clone()))?;
        registry.register(Box::new(gas_price_used_on_latest_batch.clone()))?;
        registry.register(Box::new(batcher_started.clone()))?;
        registry.register(Box::new(broken_ws_connections.clone()))?;

        let metrics_route = warp::path!("metrics")
            .and(warp::any().map(move || registry.clone()))
            .and_then(BatcherMetrics::metrics_handler);

        tokio::task::spawn(async move {
            warp::serve(metrics_route)
                .run(([0, 0, 0, 0], metrics_port))
                .await;
        });

        Ok(Self {
            open_connections,
            received_proofs,
            sent_batches,
            reverted_batches,
            batcher_started,
            gas_price_used_on_latest_batch,
            broken_ws_connections,
        })
    }

    pub async fn metrics_handler(registry: prometheus::Registry) -> Result<impl Reply, Rejection> {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();

        let mut buffer = Vec::new();
        if let Err(e) = encoder.encode(&registry.gather(), &mut buffer) {
            eprintln!("could not encode prometheus metrics: {}", e);
        };
        let res = String::from_utf8(buffer.clone())
            .inspect_err(|e| eprintln!("prometheus metrics could not be parsed correctly: {e}"))
            .unwrap_or_default();
        buffer.clear();

        Ok(res)
    }

    pub fn inc_batcher_restart(&self) {
        // Sleep for 2 seconds to allow prometheus to start and set the metrics with default intial values.
        // If prometheus is not ready, the metrics will directly be set to 1 and prometheus will not be able to display the correct increment.
        thread::sleep(Duration::from_secs(2));
        self.batcher_started.inc();
    }
}
