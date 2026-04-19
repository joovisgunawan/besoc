mod cache;
mod config;
mod db;
mod dto {
    pub mod student_dto;
}
mod error;
mod handlers {
    pub mod student_handler;
}
mod health;
mod kafka {
    pub mod producer;
}
mod models {
    pub mod student;
}
mod repositories {
    pub mod student_repository;
}
mod request_id;
mod response;
mod routes {
    pub mod student_routes;
}
mod sentry_layer;
mod services {
    pub mod student_service;
}
mod state;
mod telemetry;
// mod otel_middleware;

use axum::{routing::get, Router};
use dotenvy::dotenv;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use axum_prometheus::PrometheusMetricLayer;

use crate::{
    cache::RedisCache,
    config::Config,
    db::{connect_primary_db, connect_replica_db},
    kafka::producer::KafkaProducer,
    request_id::request_id_layer,
    routes::student_routes::student_routes,
    sentry_layer::sentry_layer,
    state::AppState,
    // otel_middleware::otel_middleware,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // 1. Load config
    let config = Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error:\n{e}");
        std::process::exit(1);
    });

    // 2. Init sentry
    let _sentry_guard = config.sentry.dsn.as_deref().map(|dsn| {
        let guard = sentry::init((
            dsn,
            sentry::ClientOptions {
                environment: Some(config.sentry.environment.clone().into()),
                release: sentry::release_name!(),
                traces_sample_rate: config.sentry.traces_sample_rate,
                ..Default::default()
            },
        ));
        sentry::configure_scope(|scope| {
            scope.set_tag("service", &config.app.app_name);
            scope.set_tag("environment", &config.sentry.environment);
        });
        guard
    });

    // 3. Init tracing + OTLP
    telemetry::init(&config.telemetry);

    tracing::info!(app = %config.app.app_name, "Starting service");
    tracing::info!(host = %config.app.host, port = %config.app.port, "Binding address");

    // 4. Init infrastructure
    let write_db = connect_primary_db(&config).await;
    tracing::info!("Database primary connections initialized");
    let read_db = connect_replica_db(&config).await;
    tracing::info!("Database replica connections initialized");

    let kafka = KafkaProducer::new(&config.kafka.broker);
    tracing::info!(broker = %config.kafka.broker, "Kafka producer initialized");

    let cache = RedisCache::new(&config.redis.url).await;
    tracing::info!("Redis cache initialized");

    // 5. Build state
    let state = AppState {
        write_db,
        read_db,
        kafka,
        cache,
        kafka_topic_student_events: config.kafka.topic_student_events.clone(),
    };

    // 6. Metrics
    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();

    // 7. Build router
    let app = Router::new()
        .merge(student_routes(state))
        .route("/metrics", get(move || async move { metrics_handle.render() }))
        .layer(axum::middleware::from_fn(request_id_layer))
        .layer(axum::middleware::from_fn(sentry_layer))
        // .layer(axum::middleware::from_fn(otel_middleware))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .layer(CorsLayer::permissive());

    // 8. Bind and serve
    let addr = format!("{}:{}", config.app.host, config.app.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Failed to bind to {addr}: {e}");
            std::process::exit(1);
        });

    tracing::info!("Server running at http://{addr}");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| {
            tracing::error!("Server error: {e}");
            std::process::exit(1);
        });

    telemetry::shutdown();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => tracing::info!("Received Ctrl+C, shutting down"),
        _ = terminate => tracing::info!("Received SIGTERM, shutting down"),
    }
}