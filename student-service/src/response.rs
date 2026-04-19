// src/response.rs
use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};  // ← add Value here

pub fn success<T: Serialize>(message: &str, data: T) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": message,
        "data": data,
    }))
}

pub fn success_paginated<T: Serialize>(
    message: &str,
    data: T,
    total: i64,
    page: i64,
    per_page: i64,
) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": message,
        "data": data,
        "meta": {
            "total": total,
            "page": page,
            "per_page": per_page,
            "total_pages": (total as f64 / per_page as f64).ceil() as i64,
        }
    }))
}