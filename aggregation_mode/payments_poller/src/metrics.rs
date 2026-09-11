use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use prometheus::{self, opts, Encoder, Gauge, Registry, TextEncoder};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct PaymentsPollerMetrics {
    pub registry: Registry,
    pub last_processed_block: Gauge,
    pub active_subscriptions: Gauge,
}

impl PaymentsPollerMetrics {
    pub fn start(metrics_port: u16) -> Result<Arc<Self>, prometheus::Error> {
        let registry = Registry::new();

        let last_processed_block = Gauge::with_opts(opts!(
            "last_processed_block",
            "Last processed block by poller"
        ))?;

        let active_subscriptions = Gauge::with_opts(opts!(
            "active_subscriptions",
            "Active payment subscriptions by poller"
        ))?;

        registry.register(Box::new(last_processed_block.clone()))?;
        registry.register(Box::new(active_subscriptions.clone()))?;

        // Arc is used because metrics are a shared resource accessed by both the background and metrics HTTP
        // server and the application code, across multiple Actix worker threads. The server outlives start(),
        // so the data must be static and safely shared between threads.
        let metrics = Arc::new(Self {
            registry,
            last_processed_block,
            active_subscriptions,
        });

        let server_metrics = metrics.clone();
        tokio::spawn(async move {
            let _ = HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(server_metrics.clone()))
                    .route(
                        "/metrics",
                        web::get().to(PaymentsPollerMetrics::metrics_handler),
                    )
            })
            .bind(("0.0.0.0", metrics_port))
            .expect("failed to bind metrics server")
            .run()
            .await;
        });

        Ok(metrics)
    }

    async fn metrics_handler(metrics: web::Data<Arc<PaymentsPollerMetrics>>) -> impl Responder {
        let encoder = TextEncoder::new();
        let metric_families = metrics.registry.gather();

        let mut buffer = Vec::new();
        if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
            tracing::error!("could not encode prometheus metrics: {e}");
        }

        HttpResponse::Ok()
            .insert_header(("Content-Type", encoder.format_type()))
            .body(buffer)
    }

    pub fn register_last_processed_block(&self, value: u64) {
        self.last_processed_block.set(value as f64);
    }

    pub fn register_active_subscriptions(&self, value: i64) {
        self.active_subscriptions.set(value as f64);
    }
}
