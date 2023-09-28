use actix_web::{ get, post, web, web::Json, HttpResponse, Responder };
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use bigdecimal::BigDecimal;

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("testing123")
}

#[derive(Deserialize, Serialize)]
pub struct Book {
    name: String,
    author: String,
    description: String,
    rating: BigDecimal,
    review: String,
    finished: bool,
}

#[post("/test/post_book")]
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
