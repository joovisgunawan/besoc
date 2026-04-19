use serde_json::json;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    cache::{student_key, students_list_key},
    dto::student_dto::{
        CreateStudentParams, CreateStudentRequest, FindAllParams,
        ListStudentQuery, StudentResponse, UpdateStudentParams, UpdateStudentRequest,
    },
    repositories::student_repository,
    state::AppState,
};

const STUDENT_TTL: u64 = 300;
const STUDENT_LIST_TTL: u64 = 60;

pub struct StudentService;

impl StudentService {
    #[instrument(skip(state, req), fields(student_number = %req.student_number))]
    pub async fn create(
        state: &AppState,
        req: CreateStudentRequest,
    ) -> Result<StudentResponse, String> {
        let student = student_repository::create(
            &state.write_db,
            CreateStudentParams {
                student_number: req.student_number,
                name: req.name,
                email: req.email,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

        let response = StudentResponse::from(student.clone());

        state.cache.set(
            &student_key(&student.uuid.to_string()),
            &response,
            STUDENT_TTL,
        ).await;
        state.cache.delete_pattern("students:list:*").await;

        if let Err(e) = state.kafka.publish(
            &state.kafka_topic_student_events,
            &student.uuid.to_string(),
            &json!({
                "event": "STUDENT_CREATED",
                "student_uuid": student.uuid,
                "student_number": student.student_number,
                "name": student.name,
                "email": student.email,
            }).to_string(),
        ).await {
            tracing::warn!(error = %e, "Failed to publish STUDENT_CREATED event");
        }

        Ok(response)
    }

    #[instrument(skip(state))]
    pub async fn get_all(
        state: &AppState,
        query: ListStudentQuery,
    ) -> Result<(Vec<StudentResponse>, i64), String> {
        let page = query.page.unwrap_or(1).max(1);
        let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
        let offset = (page - 1) * per_page;
        let cache_key = students_list_key(page, per_page);

        if let Some(cached) = state.cache.get::<(Vec<StudentResponse>, i64)>(&cache_key).await {
            tracing::info!(cache.key = %cache_key, "Cache hit");
            return Ok(cached);
        }
        tracing::info!(cache.key = %cache_key, "Cache miss");

        let (students, total) = student_repository::find_all(
            &state.read_db,
            FindAllParams { offset, limit: per_page },
        )
        .await
        .map_err(|e| e.to_string())?;

        let response: Vec<StudentResponse> = students
            .into_iter()
            .map(StudentResponse::from)
            .collect();

        state.cache.set(
            &cache_key,
            &(response.clone(), total),
            STUDENT_LIST_TTL,
        ).await;

        Ok((response, total))
    }

    #[instrument(skip(state), fields(student.uuid = %uuid))]
    pub async fn get_by_uuid(
        state: &AppState,
        uuid: Uuid,
    ) -> Result<Option<StudentResponse>, String> {
        let cache_key = student_key(&uuid.to_string());

        if let Some(cached) = state.cache.get::<StudentResponse>(&cache_key).await {
            tracing::info!(cache.key = %cache_key, "Cache hit");
            return Ok(Some(cached));
        }
        tracing::info!(cache.key = %cache_key, "Cache miss");

        let student = student_repository::find_by_uuid(&state.read_db, uuid)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(ref s) = student {
            state.cache.set(
                &cache_key,
                &StudentResponse::from(s.clone()),
                STUDENT_TTL,
            ).await;
        }

        Ok(student.map(StudentResponse::from))
    }

    #[instrument(skip(state, req), fields(student.uuid = %uuid))]
    pub async fn update(
        state: &AppState,
        uuid: Uuid,
        req: UpdateStudentRequest,
    ) -> Result<StudentResponse, String> {
        let updated = student_repository::update(
            &state.write_db,
            UpdateStudentParams {
                uuid,
                name: req.name,
                email: req.email,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

        let student = updated
            .ok_or_else(|| "not_found: student not found".to_string())?;

        let response = StudentResponse::from(student.clone());

        state.cache.set(
            &student_key(&uuid.to_string()),
            &response,
            STUDENT_TTL,
        ).await;
        state.cache.delete_pattern("students:list:*").await;

        if let Err(e) = state.kafka.publish(
            &state.kafka_topic_student_events,
            &student.uuid.to_string(),
            &json!({
                "event": "STUDENT_UPDATED",
                "student_uuid": student.uuid,
                "name": student.name,
                "email": student.email,
            }).to_string(),
        ).await {
            tracing::warn!(error = %e, "Failed to publish STUDENT_UPDATED event");
        }

        Ok(response)
    }

    #[instrument(skip(state), fields(student.uuid = %uuid))]
    pub async fn delete(state: &AppState, uuid: Uuid) -> Result<(), String> {
        let deleted = student_repository::soft_delete(&state.write_db, uuid)
            .await
            .map_err(|e| e.to_string())?;

        let student = deleted
            .ok_or_else(|| "not_found: student does not exist".to_string())?;

        state.cache.delete(&student_key(&uuid.to_string())).await;
        state.cache.delete_pattern("students:list:*").await;

        if let Err(e) = state.kafka.publish(
            &state.kafka_topic_student_events,
            &student.uuid.to_string(),
            &json!({
                "event": "STUDENT_DELETED",
                "student_uuid": student.uuid,
            }).to_string(),
        ).await {
            tracing::warn!(error = %e, "Failed to publish STUDENT_DELETED event");
        }

        Ok(())
    }
}

impl From<crate::models::student::Student> for StudentResponse {
    fn from(s: crate::models::student::Student) -> Self {
        Self {
            uuid: s.uuid,
            student_number: s.student_number,
            name: s.name,
            email: s.email,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}