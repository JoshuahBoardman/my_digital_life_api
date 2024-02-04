pub mod configuration;

use configuration::get_configuration;
use my_digital_life::{
    email_client::{EmailClient, EmailTemplate},
    startup::run,
};
use secrecy::Secret;
use sqlx::postgres::PgPoolOptions;


#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration_settings = get_configuration().expect("Failed to load config settings");
    
    println!("{:#?}", configuration_settings);
    let database_settings = &configuration_settings.database;
    let connection_pool = PgPoolOptions::new().connect_lazy_with(database_settings.with_db());

    let email_client_settings = &configuration_settings.email.email_client;
    let timeout = email_client_settings.timeout();

    let email_client = EmailClient::new(
        email_client_settings.base_url.to_owned(),
        email_client_settings.sender_address.to_owned(),
        email_client_settings.authorization_token.to_owned(),
        timeout,
    );

    let email_template_settings = &configuration_settings.email.email_template;

    let email_template = EmailTemplate {
        template_id: email_template_settings.template_id,
        template_alias: email_template_settings.template_alias.to_owned(),
    };

    let application_settings = &configuration_settings.application;
    let app_secret: Secret<String> = application_settings.secret.to_owned();
    let base_url = application_settings.base_url.to_owned();

    run(
        application_settings.get_tcp_listener(),
        connection_pool,
        email_client,
        email_template,
        app_secret,
        base_url,
    )?
    .await
}
