use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
    Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

// Error Variants.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Invalid payload.")]
    InvalidJsonBody(#[from] JsonRejection),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("A database error has occured.")]
    DatabaseError(#[from] sqlx::Error),
    #[error("An internal server error has occured.")]
    InternalError(#[from] anyhow::Error),
}

// Error Message.
#[derive(Serialize, Deserialize)]
pub struct ApiErrorResp {
    pub message: String,
}

// IntoResponse implementation for ApiError.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        // Error to be logged by tracing error! macro.
        let error_to_log = match &self {
            ApiError::InvalidJsonBody(ref err) => match err {
                JsonRejection::JsonDataError(e) => e.body_text(),
                JsonRejection::JsonSyntaxError(e) => e.body_text(),
                JsonRejection::MissingJsonContentType(_) => {
                    "Missing `Content-Type: application/json` header".to_string()
                }
                JsonRejection::BytesRejection(_) => "Failed to buffer request body".to_string(),
                _ => "Unknown error".to_string(),
            },
            ApiError::InvalidRequest(_) => format!("{}", self),
            ApiError::DatabaseError(ref err) => format!("{}", err),
            ApiError::InternalError(ref err) => format!("{}", err),
        };
        error!("ERROR -------> {}", error_to_log);

        // Error message to be sent to the API client.
        let resp = ApiErrorResp {
            message: self.to_string(),
        };

        // Status Code for error variants.
        let status = match self {
            ApiError::InvalidJsonBody(_) | ApiError::InvalidRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::DatabaseError(_) | ApiError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, Json(resp)).into_response()
    }
}
