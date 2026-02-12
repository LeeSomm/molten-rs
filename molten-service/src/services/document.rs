//! This module provides the service struct for Document entity operations.
use crate::error::ServiceError;
use molten_core::document::Document;
use molten_core::workflow::WorkflowGraph;
use molten_document::validate_document;
use molten_storage_seaorm::repo::{DocumentRepository, FormRepository, WorkflowRepository};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

/// Service for managing documents, including creation, validation, and retrieval.
///
/// This service orchestrates interactions between document data, form definitions, workflow
/// definitions, and storage.
pub struct DocumentService {
    db: DatabaseConnection,
}

impl DocumentService {
    /// Creates a new `DocumentService` instance.
    ///
    /// # Arguments
    /// * `db` - A `sea_orm::DatabaseConnection` used for database operations.
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Creates a new document, validates it against its form definition and workflow,
    /// and saves it to storage.
    ///
    /// # Arguments
    /// * `form_id` - The ID of the form definition the document adheres to.
    /// * `workflow_id` - The ID of the workflow that governs the document's lifecycle.
    /// * `data` - The actual data content of the document, as a `HashMap<String, Value>`.
    ///
    /// # Returns
    /// A `Result` which is `Ok(Document)` if the document was successfully created and
    /// persisted, or `Err(ServiceError)` if an error occurred during validation,
    /// configuration fetching, or database operations.
    ///
    /// # Steps
    /// 1. Fetch Form Definition (to check schema).
    /// 2. Fetch Workflow Definition (to find start phase).
    /// 3. Create Document instance.
    /// 4. Validate Data against Form.
    /// 5. Save to Database.
    pub async fn create_document(
        &self,
        form_id: &str,
        workflow_id: &str,
        data: HashMap<String, Value>,
    ) -> Result<Document, ServiceError> {
        // 1. Fetch Configuration
        // We need the Form to validate the data types.
        let form = FormRepository::find_by_id(&self.db, form_id)
            .await
            .map_err(ServiceError::Internal)?
            .ok_or_else(|| ServiceError::FormNotFound(form_id.to_string()))?;

        // We need the Workflow to determine where to start.
        let workflow = WorkflowRepository::find_by_id(&self.db, workflow_id)
            .await
            .map_err(ServiceError::Internal)?
            .ok_or_else(|| ServiceError::WorkflowNotFound(workflow_id.to_string()))?;

        // 2. Determine Start Phase
        // Every workflow must have exactly one "Start" phase.
        let start_phase = workflow.get_start_phase().ok_or_else(|| {
            ServiceError::WorkflowRuleViolation(molten_workflow::WorkflowError::UnknownPhase(
                "No start phase defined".into(),
            ))
        })?;

        // 3. Construct the Document
        // We generate a UUID here (or you could let the DB do it, but application-side is usually easier).
        let doc_id = Uuid::new_v4().to_string();
        let mut doc = Document::new(&doc_id, form_id, workflow_id);
        doc.current_phase = start_phase.id.clone();
        doc.data = data;

        // 4. Validate Data
        // This runs the engine we built in Task 2.1
        if let Err(validation_errors) = validate_document(&doc, &form) {
            return Err(ServiceError::DocumentValidationErrors(validation_errors));
        }

        // 5. Persist
        DocumentRepository::create(&self.db, &doc)
            .await
            .map_err(ServiceError::Internal)?;

        Ok(doc)
    }

    /// Retrieves a document by its unique identifier.
    ///
    /// # Arguments
    /// * `id` - The unique ID of the document to retrieve.
    ///
    /// # Returns
    /// A `Result` which is `Ok(Document)` if the document is found, or `Err(ServiceError)`
    /// if the document is not found or a database error occurs.
    pub async fn get_document(&self, id: &str) -> Result<Document, ServiceError> {
        DocumentRepository::find_by_id(&self.db, id)
            .await
            .map_err(ServiceError::Internal)?
            .ok_or_else(|| ServiceError::Internal(anyhow::anyhow!("Document not found")))
        // TODO: Implement specific NotFound error code
    }

    // Future: We will add `transition_document` here later
}
