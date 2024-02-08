use std::i64;

use chrono::Utc;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

use crate::model::blog::Blog;

pub struct BlogRepository<'a> {
    connection_pool: &'a PgPool,
}

impl BlogRepository<'_> {
    pub fn new<'a>(connection_pool: &'a PgPool) -> BlogRepository<'a> {
        BlogRepository { connection_pool }
    }

    pub async fn fetch_book_by_id<'a>(&self, blog_id: Uuid) -> Result<Blog, SqlxError> {
        sqlx::query_as!(
            Blog,
            r#"
                SELECT * FROM blog_posts
                WHERE id = $1 
                LIMIT 1
            "#,
            blog_id
        )
        .fetch_one(self.connection_pool)
        .await
    }

    pub async fn fetch_blogs(&self, limit: i64, offset: i64) -> Result<Vec<Blog>, SqlxError> {
        sqlx::query_as!(
            Blog,
            r#"
            SELECT * FROM blog_posts
            LIMIT $1
            OFFSET $2
        "#,
            limit,
            offset,
        )
        .fetch_all(self.connection_pool)
        .await
    }

    pub async fn post_blog<'a>(&self, blog: &'a Blog) -> Result<String, SqlxError> {
        match sqlx::query_as!(
            Blog,
            r#"
            INSERT INTO blog_posts (id, author_id, title, body, last_updated, inserted_at)
            VALUES ($1, $2, $3, $4, $5, $6) 
        "#,
            Uuid::new_v4(),
            blog.author_id,
            blog.title,
            blog.body,
            Utc::now(),
            Utc::now()
        )
        .execute(self.connection_pool)
        .await
        {
            Ok(_) => Ok("Success".to_string()),
            Err(err) => Err(err),
        }
    }
}
