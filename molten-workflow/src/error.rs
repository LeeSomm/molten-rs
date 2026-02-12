//! Defines error types specific to workflow operations within the `molten-workflow` crate.
//!
//! This module provides a comprehensive set of error variants encapsulated by
//! `WorkflowError`, each detailing specific reasons why a workflow operation failed.

use thiserror::Error;

/// Represents errors that can occur during workflow operations.
#[derive(Error, Debug, PartialEq)]
pub enum WorkflowError {
    /// Occurs when a document's associated workflow ID does not match the provided workflow ID.
    #[error(
        "Workflow ID mismatch: Document belongs to '{doc_wf}' but provided workflow is '{provided_wf}'"
    )]
    WorkflowMismatch {
        /// Document workflow ID
        doc_wf: String,
        /// Provided workflow ID
        provided_wf: String,
    },

    /// Occurs when a referenced phase ID does not exist within the workflow definition.
    #[error("Phase '{0}' does not exist in this workflow")]
    UnknownPhase(String),

    /// Occurs when an attempted transition from the current phase to a target phase is not allowed by the workflow rules.
    #[error("Invalid transition: Cannot move from '{current}' to '{target}'")]
    InvalidTransition {
        /// Current phase
        current: String,
        /// Target phase
        target: String,
    },

    /// Occurs when a document does not have a `current_phase` defined, which is required for workflow operations.
    #[error("Document has no current phase (is it a new document?)")]
    NoCurrentPhase,
}
