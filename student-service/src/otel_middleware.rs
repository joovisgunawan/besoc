use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use opentelemetry::global;
use opentelemetry::propagation::Extractor;
use tracing_opentelemetry::OpenTelemetrySpanExt;

struct HeaderExtractor<'a>(&'a axum::http::HeaderMap);

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

// ✅ FIXED SIGNATURE (Axum 0.7)
pub async fn otel_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let parent_context = global::get_text_map_propagator(|prop| {
        prop.extract(&HeaderExtractor(req.headers()))
    });

    let span = tracing::info_span!(
        "http_request",
        method = %req.method(),
        uri = %req.uri()
    );

    span.set_parent(parent_context);

    let _enter = span.enter();

    next.run(req).await
}