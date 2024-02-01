use crate::{
    email_client::{EmailClient, EmailTemplate},
    model::common::Url,
    routes::{health_check::health_check, v1::v1_scope},
};
use actix_web::{dev::Server, web::Data, App, HttpServer};
use secrecy::Secret;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    email_template: EmailTemplate,
    secret: Secret<String>,
    base_url: Url,
) -> std::io::Result<Server> {
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let email_template = Data::new(email_template);
    let secret = Data::new(secret);
    let base_url = Data::new(base_url);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(email_template.clone())
            .app_data(secret.clone())
            .app_data(base_url.clone())
            .service(health_check)
            .service(v1_scope())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
