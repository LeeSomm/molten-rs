use crate::{error::ApiError, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
};
use molten_config::{ConfigFormat, parse_content};
use molten_core::{FormDefinition, FormBuilder, document::Document};
use molten_service::ServiceError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// POST /forms
pub async fn create_form(
    State(state): State<AppState>,
    Json(builder): Json<FormBuilder>,
) -> Result<Json<FormDefinition>, ApiError> {
    let form_def: FormDefinition = builder.build()
        .map_err(|e| ServiceError::FormValidationErrors(e))?;
    
    let form = state
        .form_service
        .create_form(form_def)
        .await?;

    Ok(Json(form))
}