use serde::{Deserialize, Serialize};
use sqlx::{types::BigDecimal, FromRow};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct SavingDao {
    pub person_id: i32,
    pub savings: BigDecimal,
}
