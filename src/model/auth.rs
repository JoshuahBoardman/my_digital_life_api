use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Secret(pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,        // base_url
    pub sub: Uuid,          // User id
    pub iat: NaiveDateTime, // Token was issued timestamp
    pub exp: NaiveDateTime, // Experation timestamp
}

// May want to expand this later
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequestBody {
    pub user_email: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct VerificationCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: NaiveDateTime,
    pub inserted_at: DateTime<Utc>,
}
