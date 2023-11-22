pub mod configuration;
pub mod extractors;
pub mod model;
mod routes;

use actix_web::dev::Server;
use actix_web::{web::Data, App, HttpServer};
use routes::books::book_shelf_scope;
use routes::health_check::health_check;
use routes::auth::auth_scope;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: Data<PgPool>) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .service(health_check)
            .service(book_shelf_scope())
            .service(auth_scope())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
