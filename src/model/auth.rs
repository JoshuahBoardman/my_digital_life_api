use actix_web::{error, web::Data, Result as ActixResult};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

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
    pub async fn from_database(
        verification_code: &str,
        connection_pool: &Data<PgPool>,
    ) -> ActixResult<Self> {
        match sqlx::query_as!(
            VerificationCode,
            "
                DELETE FROM verification_codes
                WHERE code = $1
                RETURNING *;
            ",
            verification_code
        )
        .fetch_one(connection_pool.get_ref())
        .await
        {
            Ok(code) => Ok(code as VerificationCode),
            Err(err) => {
                return Err(error::ErrorInternalServerError(format!(
                    "Invaild verification code - {}",
                    err
                )))
            }
        }
    }

    pub fn verify(&self) -> ActixResult<()> {
        let naive_current_time = Utc::now().naive_utc();

        if naive_current_time > self.expires_at {
            return Err(error::ErrorUnauthorized(
                "The verification token provided has expired, please login to recieve a new token",
            ));
        }
        Ok(())
    }
}
