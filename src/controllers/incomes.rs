use crate::{
    database::DatabaseConnection,
    models::{
        self,
        income::{IncomeDao, IncomeDto},
    },
};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn post(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(income): Json<IncomeDto>,
) -> Result<Json<IncomeDao>, (StatusCode, String)> {
    let income = sqlx::query_as::<_, models::income::IncomeDao>(
        "INSERT INTO income (name, gross, rate, vat, category, type, currency, person_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
    )
    .bind(income.name)
    .bind(income.gross)
    .bind(income.rate)
    .bind(income.vat)
    .bind(income.category)
    .bind(income.income_type)
    .bind(income.currency)
    .bind(income.person_id)
    .fetch_one(&mut *conn)
    .await;

    match income {
        Ok(income) => Ok(Json(income)),
        Err(e) => {
            tracing::error!("failed to insert income: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to insert income".to_string(),
            ))
        }
    }
}

pub async fn get(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<IncomeDao>>, (StatusCode, String)> {
    let incomes = sqlx::query_as::<_, models::income::IncomeDao>("SELECT * FROM income")
        .fetch_all(&mut *conn)
        .await;

    match incomes {
        Ok(incomes) => Ok(Json(incomes)),
        Err(e) => {
            tracing::error!("failed to fetch incomes: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to fetch incomes".to_string(),
            ))
        }
    }
}

pub async fn get_one(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<Json<IncomeDao>, (StatusCode, String)> {
    let income =
        sqlx::query_as::<_, models::income::IncomeDao>("SELECT * FROM income WHERE id = $1")
            .bind(id)
            .fetch_one(&mut *conn)
            .await;

    match income {
        Ok(income) => Ok(Json(income)),
        Err(e) => {
            tracing::error!("failed to retrieve income with id {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to retrieve income".to_string(),
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
) -> Result<Json<IncomeDao>, (StatusCode, String)> {
    let income = sqlx::query_as::<_, models::income::IncomeDao>(
        "DELETE FROM income WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_one(&mut *conn)
    .await;

    match income {
        Ok(income) => Ok(Json(income)),
        Err(e) => {
            tracing::error!("failed to delete income with id {}: {}", id, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to delete income".to_string(),
            ))
        }
    }
}
