//! This module provides the API handlers for Form entity operations.
//!
//! It includes functions for creating new forms and retrieving existing ones,
//! serving as the entry point for interactions with the form service layer.
use crate::{error::ApiError, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
};
use molten_core::{FormBuilder, FormDefinition};
use molten_service::ServiceError;

/// Create a new form definition.
///
/// Accepts a [`FormBuilder`] and validates it into a finalized
/// [`FormDefinition`]. If validation succeeds, the form is
/// persisted and the stored definition is returned.
///
/// # Route
/// `POST /forms`
///
/// # Errors
/// - Returns an error if the form definition fails validation.
/// - Returns an error if persistence fails.
///
/// # Notes
/// This endpoint is intended only for creating new forms.
/// Updates to existing forms should be handled via a separate endpoint
/// to allow different validation and lifecycle rules.
pub async fn create_form(
    State(state): State<AppState>,
    Json(builder): Json<FormBuilder>,
) -> Result<Json<FormDefinition>, ApiError> {
    let form_def: FormDefinition = builder
        .build()
        .map_err(ServiceError::FormValidationErrors)?;

    let form = state.form_service.save_form(form_def).await?;

    Ok(Json(form))
}

/// Retrieve a form definition by id.
///
/// # Route
/// `GET /forms/{id}`
///
/// # Errors
/// - Returns an error if the form does not exist.
/// - Returns an error if the underlying storage operation fails.
pub async fn get_form(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<FormDefinition>, ApiError> {
    let form = state.form_service.get_form(&id).await?;
    Ok(Json(form))
}

// TODO: POST /forms/{id} for Updates
