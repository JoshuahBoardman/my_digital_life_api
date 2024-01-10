use actix_web::dev::Server;
use actix_web::{web::Data, App, HttpServer};
use crate::{
    model::common::Url,
    routes::{health_check::health_check, v1::v1_scope},
    email_client::EmailClient
};
use secrecy::Secret;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    db_pool: Data<PgPool>,
    email_client: Data<EmailClient>,
    secret: Data<Secret<String>>,
    base_rul: Data<Url>,
) -> std::io::Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(secret.clone())
            .app_data(base_rul.clone())
            .service(health_check)
            .service(v1_scope())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
