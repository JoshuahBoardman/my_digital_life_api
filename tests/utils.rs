use actix_web::{rt::spawn, web::Data};
use digital_bookshelf_api::{configuration::get_configuration, run};
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn spawn_app() -> String {
    let local_host = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0", local_host))
        .expect("Error: failed to bind to a random port");
    let port = listener.local_addr().unwrap().port();

    let database_url = get_configuration().database.get_database_url();
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Error: failed to read database configuration");
    let pool = Data::new(db_pool);

    let server = run(listener, pool).expect("Error: failed to bind address");
    let _ = spawn(server);

    format!("http://{}:{}", local_host, port)
}
