mod routes;

use std::env;

use dotenv::dotenv;

use routes::test::{send_post, test, test_JSON, test_from_path, post_book};

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use sqlx::Connection;
use sqlx::Row;
use std::error::Error;

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

    // Might need to add move to the closure
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(test)
            .service(test_JSON)
            .service(test_from_path)
            .service(send_post)
            .route("/test/post_book", web::post().to(post_book))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
