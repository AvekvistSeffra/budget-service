use crate::{
    database::DatabaseConnection,
    models::expense::{ExpenseDao, ExpenseDto},
};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn post(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(expenses): Json<ExpenseDto>,
) -> Result<Json<ExpenseDao>, (StatusCode, String)> {
    let expenses = sqlx::query_as::<_, ExpenseDao>(
        "INSERT INTO expense (name, source, currency, category, conversion_rate, person_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    )
    .bind(expenses.name)
    .bind(expenses.source)
    .bind(expenses.currency)
    .bind(expenses.category)
    .bind(expenses.conversion_rate)
    .bind(expenses.person_id)
    .fetch_one(&mut *conn)
    .await;

    match expenses {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => {
            tracing::error!("failed to insert expenses: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to insert expenses".to_string(),
            ))
        }
    }
}

pub async fn get(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<ExpenseDao>>, (StatusCode, String)> {
    let expenses = sqlx::query_as::<_, ExpenseDao>("SELECT * FROM expense")
        .fetch_all(&mut *conn)
        .await;

    match expenses {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => {
            tracing::error!("failed to fetch expenses: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to fetch expenses".to_string(),
            ))
        }
    }
}

pub async fn get_one(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<Json<ExpenseDao>, (StatusCode, String)> {
    let expense = sqlx::query_as::<_, ExpenseDao>("SELECT * FROM expense WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await;

    match expense {
        Ok(expense) => Ok(Json(expense)),
        Err(e) => {
            tracing::error!("failed to retrieve expense with id {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to retrieve expense".to_string(),
            ))
        }
    }
}

pub async fn put(
    DatabaseConnection(mut _conn): DatabaseConnection,
    Path(_id): Path<i32>,
) -> Result<Json<ExpenseDao>, (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Not implemented".to_string()))
}

pub async fn delete(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<Json<ExpenseDao>, (StatusCode, String)> {
    let expense = sqlx::query_as::<_, ExpenseDao>("DELETE FROM expense WHERE id = $1 RETURNING *")
        .bind(id)
        .fetch_one(&mut *conn)
        .await;

    match expense {
        Ok(expense) => Ok(Json(expense)),
        Err(e) => {
            tracing::error!("failed to delete expense with id {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to delete expense".to_string(),
            ))
        }
    }
}
