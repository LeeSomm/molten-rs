use crate::{error::ApiError, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
};
use molten_core::document::Document;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    pub form_id: String,
    pub workflow_id: String,
    pub data: HashMap<String, Value>,
}

// POST /documents
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

// GET /documents/:id
pub async fn get_document(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Document>, ApiError> {
    let doc = state.document_service.get_document(&id).await?;
    Ok(Json(doc))
}
