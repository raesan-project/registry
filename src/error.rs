use askama;
use axum::{self, response::IntoResponse};
use color_eyre::{self, eyre};
use hotwatch;
use serde_json;
use thiserror;
use tracing;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    HTMLTemplateRenderError(#[from] askama::Error),

    #[error("not found, {0}")]
    NotFound(String),

    #[error("invalid input, {0}")]
    InvalidInput(String),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("database error, {0}")]
    DatabaseError(String),

    #[error(transparent)]
    HotwatchError(#[from] hotwatch::Error),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

// Tell axum how to convert `Error` into a response.
impl Error {
    fn response(&self) -> axum::response::Response {
        match self {
            Self::HTMLTemplateRenderError(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "failed to render HTML tempalte".to_string(),
            )
                .into_response(),
            Self::NotFound(e) => (
                axum::http::StatusCode::NOT_FOUND,
                format!("not found: {:#?}", e.to_string()),
            )
                .into_response(),
            _ => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "something went wrong".to_string(),
            )
                .into_response(),
        }
    }
}

pub type HandlerResult<T, E = HandlerReport> = color_eyre::Result<T, E>;
pub struct HandlerReport(eyre::Report);

impl std::fmt::Debug for HandlerReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for HandlerReport
where
    E: Into<color_eyre::Report>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// Tell axum how to convert `Report` into a response.
impl axum::response::IntoResponse for HandlerReport {
    fn into_response(self) -> axum::response::Response {
        let err = self.0;
        let err_string = format!("{:?}", err);

        tracing::error!("{}", err_string);

        if let Some(err) = err.downcast_ref::<Error>() {
            return err.response();
        }

        // Fallback
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong".to_string(),
        )
            .into_response()
    }
}
