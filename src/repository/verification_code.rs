use crate::model::auth::VerificationCode;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

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
        let _ = self
            .delete_verification_code_by_user(&verification_code.user_id)
            .await?;

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

    pub async fn delete_verification_code_by_user<'a>(
        &self,
        user_id: &'a Uuid,
    ) -> Result<&str, SqlxError> {
        match sqlx::query!(
            "
                DELETE FROM verification_codes
                WHERE user_id = $1
            ",
            user_id
        )
        .execute(self.connection_pool)
        .await
        {
            Ok(_) => Ok("Success"),
            Err(err) => Err(err),
        }
    }
}
