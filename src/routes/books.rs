use crate::model::book::Book;
use actix_web::{error::Error, get, post, web, web::Json, HttpResponse, Responder, Result};
use chrono::Utc;
use sqlx::{self, PgPool};
use uuid::Uuid;

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("testing123")
}

#[get("/book-shelf/books/{bookId}")]
pub async fn get_book_by_id(
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let book_id: Uuid = path.into_inner();
    match sqlx::query_as!(
        Book,
        "
            SELECT id, name, author, description, rating, review, finished, inserted_at
            FROM bookShelf WHERE id = $1
        ",
        book_id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(book) => Ok(HttpResponse::Ok().json(book)),
        Err(_) => Ok(HttpResponse::NotFound().json("Error: book not found")),
    }
}

#[get("/book-shelf/books")]
pub async fn get_books(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    match sqlx::query_as!(
        Book,
        "
        SELECT id, name, author, description, rating, review, finished, inserted_at 
        FROM bookShelf
        "
    )
    .fetch_all(pool.as_ref())
    .await
    {
        Ok(books) => Ok(HttpResponse::Ok().json(books)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[post("/book-shelf/books")]
pub async fn post_book(book: Json<Book>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
    INSERT INTO bookShelf (id, name, author, description, rating, review, finished, inserted_at)
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
