use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
    UnprocessableEntity(Vec<String>),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, errors) = match &self {
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg.clone(), None)
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg.clone(), None)
            }
            AppError::Conflict(msg) => {
                (StatusCode::CONFLICT, msg.clone(), None)
            }
            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, msg.clone(), None)
            }
            AppError::UnprocessableEntity(errs) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation failed".to_string(),
                Some(errs.clone()),
            ),
            AppError::InternalServerError(msg) => {
                sentry::with_scope(
                    |scope| scope.set_level(Some(sentry::Level::Error)),
                    || sentry::capture_message(msg, sentry::Level::Error),
                );
                tracing::error!(error = %msg, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                    None,
                )
            }
        };

        (
            status,
            Json(json!({
                "success": false,
                "message": message,
                "errors": errors,
                "data": null,
            })),
        )
            .into_response()
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(e: validator::ValidationErrors) -> Self {
        let errors = e
            .field_errors()
            .into_iter()
            .flat_map(|(field, errs)| {
                errs.iter().map(move |err| {
                    format!(
                        "{}: {}",
                        field,
                        err.message.as_deref().unwrap_or("invalid value")
                    )
                })
            })
            .collect();
        AppError::UnprocessableEntity(errors)
    }
}