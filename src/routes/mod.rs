pub mod room;
pub mod song;
pub mod spotify;

use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

use crate::models;
use crate::models::GenericOutput;
use crate::utils::format_error;

pub fn send_data<T: Serialize>(body: T) -> HttpResponse {
    let data = GenericOutput {
        data: Some(body),
        error: None,
        success: true,
        status_code: 200,
    };

    if let Ok(result) = serde_json::to_string(&data) {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(result)
    } else {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::SERDE_ERROR)
    }
}

pub fn send_error(
    error: Box<dyn std::error::Error>,
    status_code: u16,
    error_text: &'static str,
) -> HttpResponse {
    let status = StatusCode::from_u16(status_code);
    if status.is_err() {
        eprintln!("Status code should always be a valid status code");
        return HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::INTERNAL_SERVER_ERROR);
    }

    let status = status.unwrap();
    let error = format_error(error, error_text);

    let data = GenericOutput::<()> {
        data: None,
        error: Some(error),
        success: false,
        status_code,
    };

    if let Ok(result) = serde_json::to_string(&data) {
        HttpResponse::build(status)
            .content_type("application/json")
            .body(result)
    } else {
        HttpResponse::InternalServerError()
            .content_type("application/json")
            .body(models::SERDE_ERROR)
    }
}

#[derive(Debug)]
pub struct StringError(&'static str);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StringError {}
