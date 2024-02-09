use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub inserted_at: DateTime<Utc>,
    pub last_updated: Option<DateTime<Utc>>,
}
