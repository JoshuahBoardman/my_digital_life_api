use std::i64;

use chrono::Utc;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

use crate::model::book::Book;

pub struct BookRepository<'a> {
    connection_pool: &'a PgPool,
}

impl BookRepository<'_> {
    pub fn new<'a>(connection_pool: &'a PgPool) -> BookRepository<'a> {
        BookRepository { connection_pool }
    }

    pub async fn fetch_book_by_id<'a>(&self, book_id: Uuid) -> Result<Book, SqlxError> {
        sqlx::query_as!(
            Book,
            r#"
                SELECT * FROM books 
                WHERE id = $1
                LIMIT 1

            "#,
            book_id
        )
        .fetch_one(self.connection_pool)
        .await
    }

    pub async fn fetch_books(&self, limit: i64, offset: i64) -> Result<Vec<Book>, SqlxError> {
        sqlx::query_as!(
            Book,
            r#"
                SELECT * FROM books
                LIMIT $1
                OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(self.connection_pool)
        .await
    }

    pub async fn post_book<'a>(&self, book: &'a Book) -> Result<String, SqlxError> {
        match sqlx::query!(
            r#"
                INSERT INTO books (id, name, author, description, rating, review, finished, inserted_at, last_updated)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            Uuid::new_v4(),
            book.name,
            book.author,
            book.description,
            book.rating,
            book.review,
            book.finished,
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
