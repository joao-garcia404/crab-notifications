use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct HttpError {
    pub status_code: StatusCode,
    pub message: String,
}

impl std::error::Error for HttpError {}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Http error with status: {}, message: {}",
            self.status_code, self.message
        )
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let json_response = Json(json!({
            "message": self.message,
        }));

        (self.status_code, json_response).into_response()
    }
}
