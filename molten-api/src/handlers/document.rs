//! This module provides the API handlers for Document entity operations.
//!
//! It includes functions for creating new documents and retrieving existing ones,
//! serving as the entry point for interactions with the document service layer.
use crate::{error::ApiError, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
};
use molten_core::document::Document;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// Request payload for creating a new document.
///
/// A document is created against a specific form and workflow. The `data`
/// field contains the user-provided values for the form and is validated
/// against the referenced form definition during creation.
#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    /// Unique identifier for the form that governs the document structure
    pub form_id: String,
    /// Unique identifier for the workflow that governs the document lifecycle
    pub workflow_id: String,
    /// Field values for the document, keyed by form field identifier.
    ///
    /// The contents of this map are validated against the referenced form
    /// definition (required fields, types, and constraints).
    pub data: HashMap<String, Value>,
}

/// Create a new document definition.
///
/// Accepts a [`CreateDocumentRequest`] and validates it.
/// If validation succeeds, the document is
/// persisted and the stored document is returned.
///
/// # Route
/// `POST /documents`
///
/// # Errors
/// - Returns an error if the document definition fails validation.
/// - Returns an error if persistence fails.
///
/// # Notes
/// This endpoint is intended only for creating new documents.
/// Updates to existing documents should be handled via a separate endpoint
/// to allow different validation and lifecycle rules.
pub async fn create_document(
    State(state): State<AppState>,
    Json(payload): Json<CreateDocumentRequest>,
) -> Result<Json<Document>, ApiError> {
    let doc = state
        .document_service
        .create_document(&payload.form_id, &payload.workflow_id, payload.data)
        .await?;

    Ok(Json(doc))
}

/// Retrieve a document definition by id.
///
/// # Route
/// `GET /documents/{id}`
///
/// # Errors
/// - Returns an error if the document does not exist.
/// - Returns an error if the underlying storage operation fails.
pub async fn get_document(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Document>, ApiError> {
    let doc = state.document_service.get_document(&id).await?;
    Ok(Json(doc))
}

// TODO: POST /documents/{id} for Updates
