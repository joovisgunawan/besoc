use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    dto::student_dto::{CreateStudentParams, FindAllParams, UpdateStudentParams},
    models::student::Student,
};

#[instrument(skip(db), name = "db.students.create", fields(student_number = %params.student_number))]
pub async fn create(
    db: &PgPool,
    params: CreateStudentParams,
) -> Result<Student, sqlx::Error> {
    sqlx::query_as!(
        Student,
        r#"
        INSERT INTO students (uuid, student_number, name, email)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        Uuid::new_v4(),
        params.student_number,
        params.name,
        params.email,
    )
    .fetch_one(db)
    .await
}

#[instrument(skip(db), name = "db.students.find_all", fields(limit = %params.limit, offset = %params.offset))]
pub async fn find_all(
    db: &PgPool,
    params: FindAllParams,
) -> Result<(Vec<Student>, i64), sqlx::Error> {
    let students = sqlx::query_as!(
        Student,
        r#"
        SELECT * FROM students
        WHERE deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        params.limit,
        params.offset,
    )
    .fetch_all(db)
    .await?;

    let total = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*) as "count!: i64"
        FROM students
        WHERE deleted_at IS NULL
        "#,
    )
    .fetch_one(db)
    .await?;

    Ok((students, total))
}

#[instrument(skip(db), name = "db.students.find_by_uuid", fields(student.uuid = %uuid))]
pub async fn find_by_uuid(
    db: &PgPool,
    uuid: Uuid,
) -> Result<Option<Student>, sqlx::Error> {
    sqlx::query_as!(
        Student,
        r#"
        SELECT * FROM students
        WHERE uuid = $1 AND deleted_at IS NULL
        "#,
        uuid,
    )
    .fetch_optional(db)
    .await
}

#[instrument(skip(db), name = "db.students.update", fields(student.uuid = %params.uuid))]
pub async fn update(
    db: &PgPool,
    params: UpdateStudentParams,
) -> Result<Option<Student>, sqlx::Error> {
    sqlx::query_as!(
        Student,
        r#"
        UPDATE students
        SET
            name = COALESCE($2, name),
            email = COALESCE($3, email),
            updated_at = NOW()
        WHERE uuid = $1
          AND deleted_at IS NULL
        RETURNING *
        "#,
        params.uuid,
        params.name,
        params.email,
    )
    .fetch_optional(db)
    .await
}

#[instrument(skip(db), name = "db.students.soft_delete", fields(student.uuid = %uuid))]
pub async fn soft_delete(
    db: &PgPool,
    uuid: Uuid,
) -> Result<Option<Student>, sqlx::Error> {
    sqlx::query_as!(
        Student,
        r#"
        UPDATE students
        SET deleted_at = NOW()
        WHERE uuid = $1 AND deleted_at IS NULL
        RETURNING *
        "#,
        uuid,
    )
    .fetch_optional(db)
    .await
}