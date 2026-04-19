use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{
    handlers::student_handler::{
        create_student, delete_student, get_all_students, get_student, update_student,
    },
    state::AppState,
};

pub fn student_routes(state: AppState) -> Router {
    Router::new()
        .route("/students", get(get_all_students).post(create_student))
        .route(
            "/students/:uuid",
            get(get_student).patch(update_student).delete(delete_student),
        )
        .with_state(state)
}