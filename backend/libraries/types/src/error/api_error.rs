use crate::{error::db_error::DbError, response::ApiErrorResponse};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    DbError(#[from] DbError),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    InternalServerError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::DbError(error) => error.into_response(),
            ApiError::BadRequest(error) => {
                ApiErrorResponse::send(StatusCode::BAD_REQUEST.as_u16(), Some(error))
            }
            ApiError::InternalServerError(error) => {
                ApiErrorResponse::send(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), Some(error))
            }
        }
    }
}
