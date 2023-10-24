use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct PersonDao {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonDto {
    pub name: String,
}
