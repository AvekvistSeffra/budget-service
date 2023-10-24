use serde::{Deserialize, Serialize};

pub mod expense;
pub mod income;
pub mod saving;
pub mod person;

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "income_type")]
#[sqlx(rename_all = "lowercase")]
pub enum IncomeType {
    Hourly,
    Monthly,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "currency")]
#[sqlx(rename_all = "lowercase")]
pub enum Currency {
    Sek,
    Eur,
    Usd,
}
