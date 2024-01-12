use reqwest::Client;
use serde::Serialize;
use secrecy::{Secret, ExposeSecret};

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: String,
    authorization_token: Secret<String>, 
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: String,
        authorization_token: Secret<String>,
        timeout: std::time::Duration,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();

        EmailClient {
            http_client,
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &str,
        template_id: &u32,
        template_alias: &str,
        template_model: &TemplateModel<'_>,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email/withTemplate/", &self.base_url);
        let email_body = EmailRequest {
            from: &self.sender,
            to: &recipient,
            tmeplate_id: template_id,
            template_alias: template_alias,
            template_model: template_model,
        };

        self.http_client
            .post(url)
            .header("X-Postmark-server-token", &self.authorization_token.expose_secret().to_string())
            .header("Accept", "application/json") 
            .header("Content-Type", "application/json")
            .json(&email_body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    tmeplate_id: &'a u32,
    template_alias: &'a str,
    template_model: &'a TemplateModel<'a>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateModel<'a> {
    pub magic_link: &'a str,
    pub site_name: &'a str,
    pub user_name: &'a str,
}
