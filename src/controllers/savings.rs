use crate::{database::DatabaseConnection, models::saving::SavingDao};
use axum::{http::StatusCode, Json, extract::Path};

pub async fn get(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<SavingDao>>, (StatusCode, String)> {
    let savings = sqlx::query_as::<_, SavingDao>("SELECT id AS person_id, savings(id) AS savings FROM person")
        .fetch_all(&mut *conn)
        .await;

    match savings {
        Ok(savings) => Ok(Json(savings)),
        Err(e) => {
            tracing::error!("failed to fetch savings: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to fetch savings".to_string(),
            ))
        }
    }
}

pub async fn get_one(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(id): Path<i32>,
) -> Result<Json<SavingDao>, (StatusCode, String)> {
    let savings = sqlx::query_as::<_, SavingDao>("SELECT id AS person_id, savings(id) AS savings FROM person WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await;

    match savings {
        Ok(savings) => Ok(Json(savings)),
        Err(e) => {
            tracing::error!("failed to fetch savings: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to fetch savings".to_string(),
            ))
        }
    }
}
