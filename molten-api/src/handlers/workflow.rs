use crate::{error::ApiError, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
};
use molten_core::{WorkflowBuilder, WorkflowDefinition};
use molten_service::ServiceError;

/// Create a new workflow definition.
///
/// Accepts a [`WorkflowBuilder`] and validates it into a finalized
/// [`WorkflowDefinition`]. If validation succeeds, the workflow is
/// persisted and the stored definition is returned.
///
/// # Route
/// `POST /workflows`
///
/// # Errors
/// - Returns an error if the workflow definition fails validation.
/// - Returns an error if persistence fails.
///
/// # Notes
/// This endpoint is intended only for creating new workflows.
/// Updates to existing workflows should be handled via a separate endpoint
/// to allow different validation and lifecycle rules.
pub async fn create_workflow(
    State(state): State<AppState>,
    Json(builder): Json<WorkflowBuilder>,
) -> Result<Json<WorkflowDefinition>, ApiError> {
    let workflow_def: WorkflowDefinition = builder
        .build()
        .map_err(|e| ServiceError::WorkflowValidationErrors(e))?;

    let workflow = state.workflow_service.save_workflow(workflow_def).await?;

    Ok(Json(workflow))
}

/// Retrieve a workflow definition by id.
///
/// # Route
/// `GET /workflows/{id}`
///
/// # Errors
/// - Returns an error if the workflow does not exist.
/// - Returns an error if the underlying storage operation fails.
pub async fn get_workflow(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<WorkflowDefinition>, ApiError> {
    let workflow = state.workflow_service.get_workflow(&id).await?;
    Ok(Json(workflow))
}

// TODO: POST /workflows/{id} for Updates
