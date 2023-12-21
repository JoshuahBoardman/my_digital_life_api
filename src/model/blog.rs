use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Blog {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub body: String,
    pub last_updated: DateTime<Utc>,
    pub inserted_at: DateTime<Utc>,
}
