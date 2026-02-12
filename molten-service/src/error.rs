//! Defines error types specific to service orchestration within the `molten-service` crate.
//!
//! This module provides a comprehensive set of error variants encapsulated by
//! `ServiceError`, each detailing specific reasons why a service failed.
use molten_document::DocumentValidationError;
use molten_workflow::WorkflowError;
use thiserror::Error;

/// Represents errors that can occur within the Molten service layer.
#[derive(Error, Debug)]
pub enum ServiceError {
    /// An error occurred during a database operation.
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    /// A requested form definition was not found.
    #[error("Form definition not found: {0}")]
    FormNotFound(String),

    /// A requested workflow definition was not found.
    #[error("Workflow definition not found: {0}")]
    WorkflowNotFound(String),

    /// Document validation failed, returning a list of specific errors.
    #[error("Document validation failed: {0:?}")]
    DocumentValidationErrors(Vec<DocumentValidationError>),

    /// Form validation failed, returning a detailed error structure.
    #[error("Form validation failed: {0:?}")]
    FormValidationErrors(validator::ValidationErrors),

    /// Workflow validation failed, returning a detailed error structure.
    #[error("Workflow validation failed: {0:?}")]
    WorkflowValidationErrors(validator::ValidationErrors),

    /// A rule defined in a workflow was violated.
    #[error("Workflow violation: {0}")]
    WorkflowRuleViolation(#[from] WorkflowError),

    /// An unexpected internal error occurred.
    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
