use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use tracing::{instrument, Span};
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::student_dto::{CreateStudentRequest, ListStudentQuery, UpdateStudentRequest},
    error::AppError,
    response::{success, success_paginated},
    services::student_service::StudentService,
    state::AppState,
};

#[instrument(
    skip(state, req),
    fields(
        student_number = %req.student_number,
        error = tracing::field::Empty,
        otel.status_code = tracing::field::Empty,
    )
)]
pub async fn create_student(
    State(state): State<AppState>,
    Json(req): Json<CreateStudentRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    req.validate().map_err(AppError::from)?;

    StudentService::create(&state, req)
        .await
        .map_err(|e| {
            Span::current().record("error", &e.as_str());
            Span::current().record("otel.status_code", "ERROR");
            AppError::InternalServerError(e)
        })
        .map(|student| (StatusCode::CREATED, success("student created", student)))
}

#[instrument(
    skip(state, query),
    fields(
        page = ?query.page,
        per_page = ?query.per_page,
        error = tracing::field::Empty,
        otel.status_code = tracing::field::Empty,
    )
)]
pub async fn get_all_students(
    State(state): State<AppState>,
    Query(query): Query<ListStudentQuery>,
) -> Result<Json<serde_json::Value>, AppError> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    StudentService::get_all(&state, query)
        .await
        .map_err(|e| {
            Span::current().record("error", &e.as_str());
            Span::current().record("otel.status_code", "ERROR");
            AppError::InternalServerError(e)
        })
        .map(|(students, total)| {
            success_paginated("students fetched", students, total, page, per_page)
        })
}

#[instrument(
    skip(state),
    fields(
        student.uuid = %uuid,
        error = tracing::field::Empty,
        otel.status_code = tracing::field::Empty,
    )
)]
pub async fn get_student(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    StudentService::get_by_uuid(&state, uuid)
        .await
        .map_err(|e| {
            Span::current().record("error", &e.as_str());
            Span::current().record("otel.status_code", "ERROR");
            AppError::InternalServerError(e)
        })
        .and_then(|student| match student {
            Some(s) => Ok(success("student fetched", s)),
            None => Err(AppError::NotFound("student not found".into())),
        })
}

#[instrument(
    skip(state, req),
    fields(
        student.uuid = %uuid,
        error = tracing::field::Empty,
        otel.status_code = tracing::field::Empty,
    )
)]
pub async fn update_student(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(req): Json<UpdateStudentRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    req.validate().map_err(AppError::from)?;

    StudentService::update(&state, uuid, req)
        .await
        .map_err(|e| {
            Span::current().record("otel.status_code", "ERROR");
            Span::current().record("error", &e.as_str());
            if e.contains("not_found") {
                AppError::NotFound(e)
            } else {
                AppError::InternalServerError(e)
            }
        })
        .map(|student| success("student updated", student))
}

#[instrument(
    skip(state),
    fields(
        student.uuid = %uuid,
        error = tracing::field::Empty,
        otel.status_code = tracing::field::Empty,
    )
)]
pub async fn delete_student(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    StudentService::delete(&state, uuid)
        .await
        .map_err(|e| {
            Span::current().record("otel.status_code", "ERROR");
            Span::current().record("error", &e.as_str());
            if e.contains("not_found") {
                AppError::NotFound(e)
            } else {
                AppError::InternalServerError(e)
            }
        })
        .map(|_| StatusCode::NO_CONTENT)
}