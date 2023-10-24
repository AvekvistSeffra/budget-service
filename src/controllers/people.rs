use crate::{
    database::DatabaseConnection,
    models::{
        self,
        person::{PersonDao, PersonDto},
    },
};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn post(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(person): Json<PersonDto>,
) -> Result<Json<PersonDao>, (StatusCode, String)> {
    let person = sqlx::query_as::<_, models::person::PersonDao>(
        "INSERT INTO person (name) VALUES ($1) RETURNING *",
    )
    .bind(person.name)
    .fetch_one(&mut *conn)
    .await;

    match person {
        Ok(person) => Ok(Json(person)),
        Err(e) => {
            tracing::error!("failed to insert person: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to insert person".to_string(),
            ))
        }
    }
}

pub async fn get(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<PersonDao>>, (StatusCode, String)> {
    let persons = sqlx::query_as::<_, models::person::PersonDao>("SELECT * FROM person")
        .fetch_all(&mut *conn)
        .await;

    match persons {
        Ok(persons) => Ok(Json(persons)),
        Err(e) => {
            tracing::error!("failed to fetch persons: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to fetch persons".to_string(),
            ))
        }
    }
}

pub async fn get_one(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<Json<PersonDao>, (StatusCode, String)> {
    let person =
        sqlx::query_as::<_, models::person::PersonDao>("SELECT * FROM person WHERE id = $1")
            .bind(id)
            .fetch_one(&mut *conn)
            .await;

    match person {
        Ok(person) => Ok(Json(person)),
        Err(e) => {
            tracing::error!("failed to retrieve person with id {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to retrieve person".to_string(),
            ))
        }
    }
}

pub async fn put(
    DatabaseConnection(mut _conn): DatabaseConnection,
    Path(_id): Path<i32>,
) -> Result<String, (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string()))
}

pub async fn delete(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<Json<PersonDao>, (StatusCode, String)> {
    let person = sqlx::query_as::<_, models::person::PersonDao>(
        "DELETE FROM person WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_one(&mut *conn)
    .await;

    match person {
        Ok(person) => Ok(Json(person)),
        Err(e) => {
            tracing::error!("failed to delete person with id {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to delete person".to_string(),
            ))
        }
    }
}
