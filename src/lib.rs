pub mod configuration;
pub mod extractors;
pub mod model;
mod routes;

use actix_web::dev::Server;
use actix_web::{web::Data, App, HttpServer};
use routes::books::{get_book_by_id, get_books, post_book};
use routes::health_check::health_check;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: Data<PgPool>) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .service(health_check)
            .service(post_book)
            .service(get_book_by_id)
            .service(get_books)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
