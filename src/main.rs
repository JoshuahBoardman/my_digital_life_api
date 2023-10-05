pub mod model;
mod routes;

use std::env;

use dotenv::dotenv;

use routes::books::{get_book_by_id, get_books, post_book, test};

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_host = &env::var("DB_HOST").expect("Error: DB_HOST .env variable not found");
    let db_port = &env::var("DB_PORT").expect("Error: DB_PORT .env variable not found");
    let db_name = &env::var("DB_NAME").expect("Error: DB_Name .env variable not found");
    let db_user = &env::var("DB_USER").expect("Error: DB_USER .env variable not found");
    let db_password = &env::var("DB_PASSWORD").expect("Error: DB_PASSWORD .env variable not found");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    print!("{}", database_url);
    let pool = sqlx::postgres::PgPool::connect(&database_url)
        .await
        .expect("Failed to read configuration.");
    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(test)
            .service(post_book)
            .service(get_book_by_id)
            .service(get_books)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
