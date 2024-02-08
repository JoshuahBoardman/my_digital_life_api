use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct Book {
    pub id: Option<Uuid>,
    pub name: String,
    pub author: String,
    pub description: String,
    pub rating: BigDecimal,
    pub review: Option<String>,
    pub finished: bool,
    pub inserted_at: Option<DateTime<Utc>>,
}
