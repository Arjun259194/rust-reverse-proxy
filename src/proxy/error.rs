use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde::Serialize;

#[derive(Debug)]
pub enum HandlerErr {
    NOTFOUND,
    BADREQUEST(String),
    INTERNALERROR(String),
}

#[derive(Serialize)]
struct ErrorMessage {
    error: String,
}

impl IntoResponse for HandlerErr {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::NOTFOUND => (StatusCode::NOT_FOUND, "not found".to_string()),
            Self::BADREQUEST(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::INTERNALERROR(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ErrorMessage { error: message });

        (status, body).into_response()
    }
}
