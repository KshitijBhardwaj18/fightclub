use axum::{
    response::{IntoResponse, Response},
    Json,
    http::StatusCode,
};
use serde::Serialize;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
}

impl AppError {
    pub fn new(message: impl Into<String>, status_code: StatusCode) -> Self {
        Self {
            message: message.into(),
            status_code,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            success: false,
            error: self.message,
        });

        (self.status_code, body).into_response()
    }
}