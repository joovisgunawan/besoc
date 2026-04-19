use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    runtime,
    trace::{self, Sampler},
    Resource,
};
use opentelemetry_semantic_conventions::resource;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::config::TelemetryConfig;

pub fn init(config: &TelemetryConfig) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,tower_http=info,sqlx=warn".into());

    let fmt_layer = tracing_subscriber::fmt::layer();

    if !config.enabled {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(sentry_tracing::layer())
            .init();
        return;
    }

    let resource = Resource::new(vec![
        KeyValue::new(resource::SERVICE_NAME, config.service_name.clone()),
        KeyValue::new(resource::SERVICE_VERSION, config.service_version.clone()),
    ]);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&config.otlp_endpoint),
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_resource(resource),
        )
        .install_batch(runtime::Tokio)
        .unwrap_or_else(|e| {
            eprintln!("Failed to init OTLP tracer: {e}");
            std::process::exit(1);
        });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(sentry_tracing::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    tracing::info!(
        endpoint = %config.otlp_endpoint,
        service = %config.service_name,
        "OpenTelemetry initialized"
    );
}

pub fn shutdown() {
    global::shutdown_tracer_provider();
}