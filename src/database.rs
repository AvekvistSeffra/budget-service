use axum::{extract::{FromRequestParts, FromRef}, http::{StatusCode, request::Parts}, async_trait};
use sqlx::PgPool;

pub struct DatabaseConnection(pub sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(|e| {
            tracing::error!("failed to acquire database connection: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "failed to acquire database connection".to_string())
        })?;

        Ok(DatabaseConnection(conn))
    }
}
