use reqwest::Client;
use serde::Serialize;

pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: String,
    authorization_token: String, // TODO: may want to wrap this as a secret from the secrecy
                                 // package to maximize security for the token
}

impl EmailClient {
    pub fn new(base_url: String, sender: String, authorization_token: String) -> Self {
        EmailClient {
            http_client: Client::new(),
            base_url,
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: String,
        //subject: &str,
        template_id: &u32,
        template_alias: &str,
        template_model: &TemplateModel<'_>,
        /*html_body: &str,
        text_body: &str,*/
    ) -> Result<String, reqwest::Error> {
        let url = format!("{}/email/withTemplate/", &self.base_url);
        let email_body = EmailRequest {
            from: &self.sender,
            to: &recipient,
            //subject: subject,
            tmeplate_id: template_id,
            template_alias: template_alias,
            template_model: template_model,
            /*html_body: html_body,
            text_body: text_body,*/
        };

        let builder = self
            .http_client
            .post(url)
            .header("X-Postmark-server-token", &self.authorization_token)
            .header("Accept", "application/json") //TODO: user headers instead of header
            .header("Content-Type", "application/json")
            .json(&email_body)
            .send()
            .await?
            .text()
            .await?;

        Ok(builder) // TODO: needs to have error handling
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    //subject: &'a str,
    tmeplate_id: &'a u32,
    template_alias: &'a str,
    template_model: &'a TemplateModel<'a>,
    /*html_body: &'a str,
    text_body: &'a str,*/
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateModel<'a> {
    pub magic_link: &'a str,
    pub site_name: &'a str,
    pub user_name: &'a str,
}
