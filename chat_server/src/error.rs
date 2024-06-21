use axum::{
    extract::multipart::MultipartError,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),

    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("password hash error: {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),

    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("create chat error: {0}")]
    CreateChatError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("multipart error: {0}")]
    MultipartError(#[from] MultipartError),

    #[error("create message error: {0}")]
    CreateMessageError(String),

    #[error("chat file error: {0}")]
    ChatFileError(String),

    #[error("parse header value: {0}")]
    HeaderError(#[from] header::InvalidHeaderValue),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::PasswordHashError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            Self::CreateChatError(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MultipartError(_) => StatusCode::BAD_REQUEST,
            Self::CreateMessageError(_) => StatusCode::BAD_REQUEST,
            Self::ChatFileError(_) => StatusCode::BAD_REQUEST,
            Self::HeaderError(_) => StatusCode::BAD_REQUEST,
        };

        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
