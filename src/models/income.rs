use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::BigDecimal;

use super::{IncomeType, Currency};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct IncomeDao {
    pub id: i32,
    pub name: String,
    pub gross: BigDecimal,
    pub rate: BigDecimal,
    pub vat: BigDecimal,
    pub netto: BigDecimal,
    pub category: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub income_type: IncomeType,
    pub currency: Currency,
    pub person_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IncomeDto {
    pub name: String,
    pub gross: BigDecimal,
    pub rate: BigDecimal,
    pub vat: BigDecimal,
    pub category: String,
    #[serde(rename = "type")]
    pub income_type: Option<IncomeType>,
    pub currency: Currency,
    pub person_id: i32,
}
