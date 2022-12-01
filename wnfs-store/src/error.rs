use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;
use std::error::Error;

//----------------------------------------------------------------
// Types
//----------------------------------------------------------------

pub enum AppError {
    InternalServerError(anyhow::Error),
    ValidationError,
}

#[derive(Debug, thiserror::Error)]
pub enum InternalError {
    #[error("Cannot fetch from store")]
    NotFoundInStore,
    #[error("Invalid data store kind: {0}")]
    InvalidDataStoreKind(String),
}

//----------------------------------------------------------------
// Implementations
//----------------------------------------------------------------

impl From<anyhow::Error> for AppError {
    fn from(inner: anyhow::Error) -> Self {
        AppError::InternalServerError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::InternalServerError(inner) => {
                tracing::debug!("stacktrace: {}", inner.backtrace());
                (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong")
            }
            AppError::ValidationError => (StatusCode::UNPROCESSABLE_ENTITY, "validation errors"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

//----------------------------------------------------------------
// Functions
//----------------------------------------------------------------

pub(crate) fn anyhow<E>(err: E) -> AppError
where
    E: Error + Send + Sync + 'static,
{
    AppError::InternalServerError(err.into())
}
