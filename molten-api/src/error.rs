//! Contains error types and their HTTP response conversions for the Molten API.
//!
//! This module defines `ApiError` for handling various application-specific errors
//! originating from service and configuration layers, converting them into
//! appropriate HTTP status codes and JSON responses. It also includes `BuildError`
//! for errors encountered during application startup.
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use molten_config::ConfigError;
use molten_service::ServiceError;
use serde_json::json;
use thiserror::Error;

/// A wrapper to allow us to implement Http responses for the API
#[derive(Error, Debug)]
pub enum ApiError {
    /// Errors generated from calling molten-service functions
    #[error("molten-service error: {0:?}")]
    Service(#[from] ServiceError),
    /// Errors generated from calling molten-config functions
    #[error("molten-config error: {0:?}")]
    Config(#[from] ConfigError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            // 404 Not Found
            ApiError::Service(ServiceError::FormNotFound(id)) => {
                (StatusCode::NOT_FOUND, format!("Form '{}' not found", id))
            }
            ApiError::Service(ServiceError::WorkflowNotFound(id)) => (
                StatusCode::NOT_FOUND,
                format!("Workflow '{}' not found", id),
            ),

            // 400 Bad Request (Validation)
            ApiError::Service(ServiceError::DocumentValidationErrors(errs)) => {
                // Return the detailed list of validation failures
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Document Validation Failed", "details": errs })),
                )
                    .into_response();
            }
            ApiError::Service(ServiceError::FormValidationErrors(e)) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            ApiError::Service(ServiceError::WorkflowValidationErrors(e)) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            ApiError::Config(ConfigError::ValidationErrors(e)) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            ApiError::Service(ServiceError::WorkflowRuleViolation(e)) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            ApiError::Config(ConfigError::JsonError(e)) => (StatusCode::BAD_REQUEST, e.to_string()),

            // 500 Internal Server Error
            ApiError::Service(ServiceError::DatabaseError(e)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            ApiError::Service(ServiceError::Internal(e)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            ApiError::Config(e) => {
                tracing::error!("Unhandled ConfigError in API layer: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Internal server error" })),
                )
                    .into_response();
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

/// Represents errors that can occur during the API server's startup phase.
/// These errors typically relate to database connection issues or I/O operations
/// essential for initializing the application.
#[derive(Debug, Error)]
pub enum BuildError {
    /// Errors generated during startup from database operations
    #[error("database error during startup")]
    Database(#[from] sea_orm::DbErr),

    /// Errors generated during startup from running the application
    #[error("I/O error during startup")]
    Io(#[from] std::io::Error),
}
