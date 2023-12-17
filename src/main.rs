pub mod configuration;

use actix_web::web::Data;
use configuration::get_configuration;
use digital_bookshelf_api::{email_client::EmailClient, model::auth::Secret, run};
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration_settings = get_configuration();

    let database_settings = &configuration_settings.database;
    let database_url = &database_settings.get_database_url();

    let connection_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to read configuration");
    let connection_pool = Data::new(connection_pool);

    let email_client_settings = &configuration_settings.email_client;

    println!("{:#?}", email_client_settings.base_url);

    let email_client = Data::new(EmailClient::new(
        email_client_settings.base_url.to_owned(),
        email_client_settings.sender_address.to_owned(),
        email_client_settings.authorization_token.to_owned(),
    ));

    let application_settings = &configuration_settings.application;
    let app_secret = Data::new(Secret(application_settings.secret.to_owned()));
    let base_url = Data::new(application_settings.base_url.to_owned());

    run(
        application_settings.get_tcp_listener(),
        connection_pool,
        email_client,
        app_secret,
        base_url,
    )?
    .await
}
