use crate::response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("{0}")]
    SomethingWentWrong(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),
}

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        let status_code = match self {
            DbError::SomethingWentWrong(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::ValidationError(_) => StatusCode::BAD_REQUEST,
            DbError::NotFound(_) => StatusCode::NOT_FOUND,
            DbError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
