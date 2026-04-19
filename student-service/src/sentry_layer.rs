// use axum::{body::Body, extract::Request, http::HeaderValue, middleware::Next, response::Response};
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn sentry_layer(req: Request<Body>, next: Next) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    sentry::add_breadcrumb(sentry::Breadcrumb {
        ty: "http".to_string(),
        category: Some("request".to_string()),
        message: Some(format!("{method} {path}")),
        level: sentry::Level::Info,
        ..Default::default()
    });

    sentry::configure_scope(|scope| {
        scope.set_tag("http.method", &method);
        scope.set_tag("http.path", &path);
    });

    let response = next.run(req).await;
    let status = response.status().as_u16();
    let duration_ms = start.elapsed().as_millis();

    sentry::add_breadcrumb(sentry::Breadcrumb {
        ty: "http".to_string(),
        category: Some("response".to_string()),
        message: Some(format!("{method} {path} -> {status} ({duration_ms}ms)")),
        level: if status >= 500 {
            sentry::Level::Error
        } else if status >= 400 {
            sentry::Level::Warning
        } else {
            sentry::Level::Info
        },
        ..Default::default()
    });

    if duration_ms > 500 {
        sentry::with_scope(
            |scope| {
                scope.set_tag("http.method", &method);
                scope.set_tag("http.path", &path);
                scope.set_tag("http.status", status.to_string());
                scope.set_level(Some(sentry::Level::Warning));
            },
            || sentry::capture_message(
                &format!("Slow request: {method} {path} took {duration_ms}ms"),
                sentry::Level::Warning,
            ),
        );
    }

    response
}