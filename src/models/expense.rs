use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::BigDecimal};

use super::Currency;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ExpenseDao {
    pub id: i32,
    pub name: String,
    pub source: BigDecimal,
    pub calculated: BigDecimal,
    pub currency: Currency,
    pub category: String,
    pub conversion_rate: BigDecimal,
    pub person_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpenseDto {
    pub name: String,
    pub source: BigDecimal,
    pub currency: Currency,
    pub category: String,
    pub conversion_rate: BigDecimal,
    pub person_id: i32,
}
