use poem_openapi::{ApiResponse, payload::PlainText};
use thiserror::Error;

#[derive(Debug, Error, ApiResponse)]
pub enum Error {
    /// A database error
    #[error("Database error: {}", .0.0)]
    #[oai(status = 500)]
    DatabaseError(PlainText<String>),

    /// An internal error
    #[error("Internal server error")]
    #[oai(status = 500)]
    InternalServerError,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::DatabaseError(PlainText(err.to_string()))
    }
}

impl From<eyre::Error> for Error {
    fn from(err: eyre::Error) -> Self {
        log::error!("{err}");

        Self::InternalServerError
    }
}
