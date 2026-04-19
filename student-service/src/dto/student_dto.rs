use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateStudentRequest {
    #[validate(length(min = 1, max = 50, message = "student_number must be 1-50 chars"))]
    pub student_number: String,

    #[validate(length(min = 1, max = 255, message = "name must be 1-255 chars"))]
    pub name: String,

    #[validate(email(message = "email must be a valid email address"))]
    #[validate(length(max = 255, message = "email must be under 255 chars"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateStudentRequest {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 chars"))]
    pub name: Option<String>,

    #[validate(email(message = "email must be a valid email address"))]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListStudentQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentResponse {
    pub uuid: Uuid,
    pub student_number: String,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct CreateStudentParams {
    pub student_number: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct UpdateStudentParams {
    pub uuid: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug)]
pub struct FindAllParams {
    pub offset: i64,
    pub limit: i64,
}