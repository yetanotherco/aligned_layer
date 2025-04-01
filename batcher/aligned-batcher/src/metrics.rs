use std::{thread, time::Duration};

// Prometheus
use prometheus::{
    core::{AtomicF64, GenericCounter},
    opts, register_counter, register_int_counter, register_int_counter_vec, register_int_gauge,
    IntCounter, IntCounterVec, IntGauge,
};

use warp::{Filter, Rejection, Reply};

#[derive(Clone, Debug)]
pub struct BatcherMetrics {
    pub open_connections: IntGauge,
    pub received_proofs: IntCounter,
    pub sent_batches: IntCounter,
    pub reverted_batches: IntCounter,
    pub canceled_batches: IntCounter,
    pub user_errors: IntCounterVec,
    pub batcher_started: IntCounter,
    pub gas_price_used_on_latest_batch: IntGauge,
    pub broken_ws_connections: IntCounter,
    pub queue_len: IntGauge,
    pub queue_size_bytes: IntGauge,
    pub s3_duration: IntGauge,
    pub create_new_task_duration: IntGauge,
    pub cancel_create_new_task_duration: IntGauge,
    pub batcher_gas_cost_create_task_total: GenericCounter<AtomicF64>,
    pub batcher_gas_cost_cancel_task_total: GenericCounter<AtomicF64>,
}

impl BatcherMetrics {
    pub fn start(metrics_port: u16) -> anyhow::Result<Self> {
        let registry = prometheus::Registry::new();

        let open_connections =
            register_int_gauge!(opts!("open_connections_count", "Open Connections"))?;
        let received_proofs =
            register_int_counter!(opts!("received_proofs_count", "Received Proofs"))?;
        let sent_batches = register_int_counter!(opts!("sent_batches_count", "Sent Batches"))?;
        let reverted_batches =
            register_int_counter!(opts!("reverted_batches_count", "Reverted Batches"))?;
        let canceled_batches =
            register_int_counter!(opts!("canceled_batches_count", "Canceled Batches"))?;
        let user_errors = register_int_counter_vec!(
            opts!("user_errors_count", "User Errors"),
            &["error_type", "proving_system"]
        )?;
        let batcher_started =
            register_int_counter!(opts!("batcher_started_count", "Batcher Started"))?;
        let gas_price_used_on_latest_batch =
            register_int_gauge!(opts!("gas_price_used_on_latest_batch", "Gas Price"))?;
        let broken_ws_connections = register_int_counter!(opts!(
            "broken_ws_connections_count",
            "Broken websocket connections"
        ))?;
        let queue_len = register_int_gauge!(opts!("queue_len", "Amount of proofs in the queue"))?;
        let queue_size_bytes = register_int_gauge!(opts!(
            "queue_size_bytes",
            "Accumulated size in bytes of all proofs in the queue"
        ))?;
        let s3_duration = register_int_gauge!(opts!("s3_duration", "S3 Duration"))?;
        let create_new_task_duration = register_int_gauge!(opts!(
            "create_new_task_duration",
            "Create New Task Duration"
        ))?;
        let cancel_create_new_task_duration = register_int_gauge!(opts!(
            "cancel_create_new_task_duration",
            "Cancel create New Task Duration"
        ))?;

        let batcher_gas_cost_create_task_total: GenericCounter<AtomicF64> =
            register_counter!(opts!(
                "batcher_gas_cost_create_task_total",
                "Batcher Gas Cost Create Task Total"
            ))?;
        let batcher_gas_cost_cancel_task_total: GenericCounter<AtomicF64> =
            register_counter!(opts!(
                "batcher_gas_cost_cancel_task_total",
                "Batcher Gas Cost Cancel Task Total"
            ))?;

        registry.register(Box::new(open_connections.clone()))?;
        registry.register(Box::new(received_proofs.clone()))?;
        registry.register(Box::new(sent_batches.clone()))?;
        registry.register(Box::new(reverted_batches.clone()))?;
        registry.register(Box::new(canceled_batches.clone()))?;
        registry.register(Box::new(user_errors.clone()))?;
        registry.register(Box::new(gas_price_used_on_latest_batch.clone()))?;
        registry.register(Box::new(batcher_started.clone()))?;
        registry.register(Box::new(broken_ws_connections.clone()))?;
        registry.register(Box::new(queue_len.clone()))?;
        registry.register(Box::new(queue_size_bytes.clone()))?;
        registry.register(Box::new(s3_duration.clone()))?;
        registry.register(Box::new(create_new_task_duration.clone()))?;
        registry.register(Box::new(cancel_create_new_task_duration.clone()))?;
        registry.register(Box::new(batcher_gas_cost_create_task_total.clone()))?;
        registry.register(Box::new(batcher_gas_cost_cancel_task_total.clone()))?;

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
            canceled_batches,
            user_errors,
            batcher_started,
            gas_price_used_on_latest_batch,
            broken_ws_connections,
            queue_len,
            queue_size_bytes,
            s3_duration,
            create_new_task_duration,
            cancel_create_new_task_duration,
            batcher_gas_cost_create_task_total,
            batcher_gas_cost_cancel_task_total,
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

    pub fn user_error(&self, label_values: &[&str]) {
        self.user_errors.with_label_values(label_values).inc();
    }

    pub fn update_queue_metrics(&self, queue_len: i64, queue_size: i64) {
        self.queue_len.set(queue_len);
        self.queue_size_bytes.set(queue_size);
    }
}
