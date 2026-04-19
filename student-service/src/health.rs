use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::state::AppState;

pub async fn health_live() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

pub async fn health_ready(
    State(state): State<AppState>,
) -> (StatusCode, Json<Value>) {
    let db_ok = sqlx::query("SELECT 1")
        .execute(&state.read_db)
        .await
        .is_ok();

    let redis_ok = state.cache.ping().await;

    let ready = db_ok && redis_ok;

    let status = if ready {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status, Json(json!({
        "status": if ready { "ready" } else { "not ready" },
        "checks": {
            "database": if db_ok { "ok" } else { "fail" },
            "redis": if redis_ok { "ok" } else { "fail" },
        }
    })))
}