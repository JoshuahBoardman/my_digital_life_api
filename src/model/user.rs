use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub user_name: String, //TODO: Sanatize this
    pub email: String,     // TODO: Valideate this and Sanatize it
    pub inserted_at: DateTime<Utc>,
}

impl User {}
