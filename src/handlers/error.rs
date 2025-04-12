use axum::{self, response::IntoResponse};
use color_eyre::{self, eyre};
use tracing;

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

        if let Some(err) = err.downcast_ref::<HandlerError>() {
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

#[derive(thiserror::Error, Debug)]
pub(crate) enum HandlerError {
    #[error("failed to render HTML template, error: {0}")]
    HTMLTemplateRender(String),
}

// Tell axum how to convert `HandlerError` into a response.
impl HandlerError {
    fn response(&self) -> axum::response::Response {
        match self {
            Self::HTMLTemplateRender(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "failed to render HTML tempalte".to_string(),
            )
                .into_response(),
        }
    }
}
