use crate::server;
use axum::{self, response::IntoResponse};
use color_eyre::{self, eyre};
use diesel;
use hotwatch;
use leptos::prelude::RenderHtml;
use r2d2;
use serde_json;
use thiserror;
use tracing;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("not found, {0}")]
    NotFound(String),

    #[error("invalid input, {0}")]
    InvalidInput(String),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    Utf8DecodeError(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    HotwatchError(#[from] hotwatch::Error),

    #[error("database error, {0}")]
    DatabaseError(String),

    #[error(transparent)]
    Other(#[from] eyre::Report),
}

// this is to convert diesel::result::Error and r2d2::Error both into DatabaseError automatically because at some places
// the map_err() method does not work properly for some reason
impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}
impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Error::DatabaseError(e.to_string())
    }
}

// Tell axum how to convert `Error` into a response.
impl Error {
    fn response(&self) -> axum::response::Response {
        match self {
            Self::NotFound(e) => (
                axum::http::StatusCode::NOT_FOUND,
                format!("not found: {:#?}", e.to_string()),
            )
                .into_response(),
            _ => {
                let html = server::web::pages::error_page::ErrorPage(
                    server::web::pages::error_page::ErrorPageProps {
                        status_code: 500.to_string(),
                        error_message: String::from("internal server error"),
                    },
                )
                .to_html();

                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    [(
                        axum::http::header::CONTENT_TYPE,
                        String::from("text/html; charset=utf-8"),
                    )],
                    html,
                )
                    .into_response();
            }
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

        let html = server::web::pages::error_page::ErrorPage(
            server::web::pages::error_page::ErrorPageProps {
                status_code: 500.to_string(),
                error_message: String::from("internal server error"),
            },
        )
        .to_html();

        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            [(
                axum::http::header::CONTENT_TYPE,
                String::from("text/html; charset=utf-8"),
            )],
            html,
        )
            .into_response();
    }
}
