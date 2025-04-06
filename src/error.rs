use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Failed to create web state for the server, Error: {0}")]
    WebStateError(String),
}
