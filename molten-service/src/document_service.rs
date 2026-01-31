use crate::error::ServiceError;
use molten_core::document::Document;
use molten_core::workflow::WorkflowGraph;
use molten_document::validate_document;
use molten_storage_seaorm::repo::{DocumentRepository, FormRepository, WorkflowRepository};
use sea_orm::DatabaseConnection;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

pub struct DocumentService {
    db: DatabaseConnection,
}

impl DocumentService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Creates a new document, validates it, and saves it to storage.
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
        let start_phase = workflow
            .get_start_phase()
            .ok_or_else(|| ServiceError::WorkflowRuleViolation(
                molten_workflow::WorkflowError::UnknownPhase("No start phase defined".into())
            ))?;

        // 3. Construct the Document
        // We generate a UUID here (or you could let the DB do it, but application-side is usually easier).
        let doc_id = Uuid::new_v4().to_string();
        let mut doc = Document::new(&doc_id, form_id, workflow_id);
        doc.current_phase = start_phase.id.clone();
        doc.data = data;

        // 4. Validate Data
        // This runs the engine we built in Task 2.1
        if let Err(validation_errors) = validate_document(&doc, &form) {
            return Err(ServiceError::ValidationErrors(validation_errors));
        }

        // 5. Persist
        DocumentRepository::create(&self.db, &doc)
            .await
            .map_err(|e| ServiceError::Internal(e))?;

        Ok(doc)
    }

    /// Retrieves a document by ID.
    pub async fn get_document(&self, id: &str) -> Result<Document, ServiceError> {
        DocumentRepository::find_by_id(&self.db, id)
            .await
            .map_err(ServiceError::Internal)?
            .ok_or_else(|| ServiceError::Internal(anyhow::anyhow!("Document not found"))) 
            // Note: In a real API, you'd want a specific NotFound error code
    }
    
    // Future: We will add `transition_document` here later
}