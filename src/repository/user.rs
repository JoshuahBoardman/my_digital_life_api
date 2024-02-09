use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

use crate::model::user::User;
pub struct UserRepository<'a> {
    connection_pool: &'a PgPool,
}

impl UserRepository<'_> {
    pub fn new<'a>(connection_pool: &'a PgPool) -> UserRepository<'a> {
        UserRepository { connection_pool }
    }

    pub async fn fetch_user_by_id<'a>(&self, user_id: &'a Uuid) -> Result<User, SqlxError> {
        sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                WHERE id = $1; 
            "#,
            user_id
        )
        .fetch_one(self.connection_pool)
        .await
    }

    pub async fn fetch_user_by_email<'a>(&self, user_email: &'a str) -> Result<User, SqlxError> {
        sqlx::query_as!(
            User,
            r#"
                SELECT * FROM users
                WHERE email = $1
                LIMIT 1
            "#,
            user_email
        )
        .fetch_one(self.connection_pool)
        .await
    }
}
