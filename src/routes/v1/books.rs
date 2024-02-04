use crate::model::book::Book;
use crate::{errors::JsonError, extractors::authentication_token::AuthenticationToken};
use actix_web::{get, post, web, web::Json, HttpResponse, Result, Scope};
use chrono::Utc;
use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

pub fn book_shelf_scope() -> Scope {
    web::scope("/books")
        .service(get_book_by_id)
        .service(get_books)
        .service(post_book)
}

#[get("/{bookId}")]
pub async fn get_book_by_id(
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, JsonError> {
    let book_id: Uuid = path.into_inner();
    match sqlx::query_as!(
        Book,
        r#"
            SELECT id, name, author, description, rating, review, finished, inserted_at
            FROM books WHERE id = $1
        "#,
        book_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(book) => Ok(HttpResponse::Ok().json(book)),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to fetch requested record - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR, //TODO: change error code set
        }),
    }
}

#[get("")]
pub async fn get_books(pool: web::Data<PgPool>) -> Result<HttpResponse, JsonError> {
    match sqlx::query_as!(
        Book,
        r#"
        SELECT id, name, author, description, rating, review, finished, inserted_at 
        FROM books
        "#
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(books) => Ok(HttpResponse::Ok().json(books)),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to fetch requested records - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

#[post("")]
pub async fn post_book(
    book: Json<Book>,
    pool: web::Data<PgPool>,
    _: AuthenticationToken,
) -> Result<HttpResponse, JsonError> {
    match sqlx::query!(
        r#"
    INSERT INTO books (id, name, author, description, rating, review, finished, inserted_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        Uuid::new_v4(),
        book.name,
        book.author,
        book.description,
        book.rating,
        book.review,
        book.finished,
        Utc::now(),
    )
    .execute(pool.as_ref())
    .await
    {
        Ok(_) => Ok(HttpResponse::Ok().json("Success")),
        Err(err) => Err(JsonError {
            response_message: format!("Error: Failed to post the requested data - {}", err),
            error_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}
