use std::i64;

use sqlx::{Error as SqlxError, PgPool};

use crate::model::auth::VerificationCode;

pub struct VerificationCodeRepository<'a> {
    connection_pool: &'a PgPool,
}

impl VerificationCodeRepository<'_> {
    pub fn new<'a>(connection_pool: &'a PgPool) -> VerificationCodeRepository<'a> {
        VerificationCodeRepository { connection_pool }
    }

    pub async fn fetch_verification_code<'a>(
        &self,
        verification_code: &'a str,
    ) -> Result<VerificationCode, SqlxError> {
        sqlx::query_as!(
            VerificationCode,
            "
                DELETE FROM verification_codes
                WHERE code = $1
                RETURNING *;
            ",
            verification_code
        )
        .fetch_one(self.connection_pool)
        .await
    }

    pub async fn post_verification_code<'a>(
        &self,
        verification_code: &'a VerificationCode,
    ) -> Result<&str, SqlxError> {
        //TODO: check if there is already a code and delete the previous one if there is

        match sqlx::query!(
            r#"
            INSERT INTO verification_codes (id, code, expires_at, user_id, inserted_at) 
            VALUES ($1, $2, $3, $4, $5)
            "#,
            verification_code.id,
            verification_code.code,
            verification_code.expires_at,
            verification_code.user_id,
            verification_code.inserted_at,
        )
        .execute(self.connection_pool)
        .await
        {
            Ok(_) => Ok("Success"),
            Err(err) => Err(err),
        }
    }
}
