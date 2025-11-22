use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

/// Application-wide error type
#[derive(Error, Debug)]
pub enum AppError {
    /// Template rendering error
    #[error("Failed to render template: {0}")]
    Template(#[from] askama::Error),

    /// Storage/IO error
    #[error("Storage error: {0}")]
    Storage(#[from] std::io::Error),

    /// JSON serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Network binding error
    #[error("Failed to bind to address: {0}")]
    Bind(std::io::Error),

    /// Server startup error
    #[error("Server error: {0}")]
    Server(std::io::Error),
}

/// Implement IntoResponse for AppError to convert errors into HTTP responses
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Template(ref e) => {
                tracing::error!("Template error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to render page".to_string(),
                )
            }
            AppError::Storage(ref e) => {
                tracing::error!("Storage error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to save data".to_string(),
                )
            }
            AppError::Serialization(ref e) => {
                tracing::error!("Serialization error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Data processing error".to_string(),
                )
            }
            AppError::Bind(ref e) => {
                tracing::error!("Bind error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server configuration error".to_string(),
                )
            }
            AppError::Server(ref e) => {
                tracing::error!("Server error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Server error".to_string(),
                )
            }
        };

        let body = Html(format!(
            "<h1>Error {}</h1><p>{}</p>",
            status.as_u16(),
            error_message
        ));

        (status, body).into_response()
    }
}

/// Convenience type alias for Results in this application
pub type Result<T> = std::result::Result<T, AppError>;
