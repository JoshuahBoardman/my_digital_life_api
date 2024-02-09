use chrono:: Duration;
use chrono::{DateTime, NaiveDateTime, Utc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::errors::JsonError;

/*#[derive(Serialize, Deserialize)]
pub struct Secret(pub String);*/

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub exp: usize,         // Experation timestamp
    pub iss: String,        // base_url
    pub sub: Uuid,          // User id
    pub iat: NaiveDateTime, // Token was issued timestamp
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

impl VerificationCode {
    pub fn new(user_id: Uuid) -> Self {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        let inserted_at: DateTime<Utc> = Utc::now();

         VerificationCode {
            id: Uuid::new_v4(),
            user_id: user_id,
            code: rand_string.to_owned(),
            expires_at: (inserted_at + Duration::hours(1)).naive_utc(),
            inserted_at,
        }
    }

    pub fn verify(&self) -> Result<(), JsonError> {
        let naive_current_time = Utc::now().naive_utc();

        if naive_current_time > self.expires_at {
            return Err(JsonError {
                response_message: "The verification token provided has expired, please login to recieve a new token".to_string(),
                error_code: StatusCode::UNAUTHORIZED,
            });
        }
        Ok(())
    }
}
