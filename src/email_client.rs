use reqwest::Client;
use serde::Serialize;

pub struct EmailClient {
    http_client: Client,
    base_url: reqwest::Url,
    sender: String,
    authorization_token: String, // TODO: may want to wrap this as a secret from the secrecy
                                 // package to maximize security for the token
}

impl EmailClient {
    pub fn new(base_url: String, sender: String, authorization_token: String) -> Self {
        EmailClient {
            http_client: Client::new(),
            base_url: reqwest::Url::parse(&base_url).expect("Failed to parse the base_url"),
            sender,
            authorization_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: String,
        subject: String,
        html_body: String,
        text_body: String,
    ) -> Result<(), String> {
        let url = self.base_url.join("/email");
        let email_body = EmailRequest {
            from: self.sender,
            to: recipient,
            subject: subject,
            html_body: html_body,
            text_body: text_body,
        };
        let builder = self.http_client.post(&url)
            .header("X-Postmark-server-token", &self.authorization_token)
            .json(&email_body)
            .send()
            .await
            .expect("Error: failled to submit");
    }

}

#[derive(Serialize)]
pub struct EmailRequest {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}
