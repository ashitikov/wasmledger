use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::time::Duration;

/// Errors that can occur during module execution
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Module compilation failed: {0}")]
    CompilationError(String),

    #[error("Module validation failed: {0}")]
    ValidationError(String),

    #[error("Module instantiation failed: {0}")]
    InstantiationError(String),

    #[error("Module execution trapped: {0}")]
    Trap(String),

    #[error("Module execution timed out after {0:?}")]
    Timeout(Duration),

    #[error("Fuel exhausted (limit: {0})")]
    FuelExhausted(u64),

    #[error("Dependency not found: {0}")]
    DependencyNotFound(String),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl ExecutionError {
    /// Get the appropriate HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::CompilationError(_) | Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::Timeout(_) => StatusCode::REQUEST_TIMEOUT,
            Self::FuelExhausted(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::DependencyNotFound(_) => StatusCode::BAD_REQUEST,
            Self::InstantiationError(_) | Self::Trap(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ExecutionError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let message = self.to_string();

        (status, Json(json!({ "error": message }))).into_response()
    }
}
