use molten_document::DocumentValidationError;
use molten_workflow::WorkflowError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),

    #[error("Form definition not found: {0}")]
    FormNotFound(String),

    #[error("Workflow definition not found: {0}")]
    WorkflowNotFound(String),

    #[error("Document validation failed: {0:?}")]
    DocumentValidationErrors(Vec<DocumentValidationError>),

    #[error("Form validation failed: {0:?}")]
    FormValidationErrors(validator::ValidationErrors),

    #[error("Workflow validation failed: {0:?}")]
    WorkflowValidationErrors(validator::ValidationErrors),

    #[error("Workflow violation: {0}")]
    WorkflowRuleViolation(#[from] WorkflowError),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
