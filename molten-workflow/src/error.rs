use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum WorkflowError {
    #[error(
        "Workflow ID mismatch: Document belongs to '{doc_wf}' but provided workflow is '{provided_wf}'"
    )]
    WorkflowMismatch { doc_wf: String, provided_wf: String },

    #[error("Phase '{0}' does not exist in this workflow")]
    UnknownPhase(String),

    #[error("Invalid transition: Cannot move from '{current}' to '{target}'")]
    InvalidTransition { current: String, target: String },

    #[error("Document has no current phase (is it a new document?)")]
    NoCurrentPhase,
}
