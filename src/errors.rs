use actix_web::{HttpResponse, ResponseError};
use reqwest::StatusCode;
use std::fmt::Display;

#[derive(Debug)]
pub struct JsonError {
    pub response_message: String,
    pub error_code: StatusCode,
}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Status Code: {}, Message: {}",
            self.error_code, self.response_message
        )
    }
}

impl ResponseError for JsonError {
    fn status_code(&self) -> StatusCode {
        self.error_code
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(&self.response_message)
    }
}
