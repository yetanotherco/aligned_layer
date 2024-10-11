use opentelemetry::{
    global,
    trace::{TraceError, TracerProvider},
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime,
    trace::{self as sdktrace, Tracer},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};

fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ],
        SCHEMA_URL,
    )
}

pub fn init_tracer(open_telemetry_jeager_url: &str) -> Result<Tracer, TraceError> {
    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(sdktrace::Config::default().with_resource(resource()))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(open_telemetry_jeager_url),
        )
        .install_batch(runtime::Tokio)?;
    global::set_tracer_provider(provider.clone());
    Ok(provider.tracer("aligned-batcher-otel-subscriber"))
}
