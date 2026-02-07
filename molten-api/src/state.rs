use molten_service::{DocumentService, FormService, WorkflowService};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// The shared state accessible by all request handlers.
/// We wrap it in Arc for cheap cloning across threads.
#[derive(Clone)]
pub struct AppState {
    /// Database Connection
    pub db: DatabaseConnection,
    /// Smart pointer to document orchestration service
    pub document_service: Arc<DocumentService>,
    /// Smart pointer to form orchestration service
    pub form_service: Arc<FormService>,
    /// Smart pointer to workflow orchestration service
    pub workflow_service: Arc<WorkflowService>,
}

impl AppState {
    /// Generates an instance of AppState
    pub fn new(db: DatabaseConnection) -> Self {
        let document_service = DocumentService::new(db.clone());
        let form_service = FormService::new(db.clone());
        let workflow_service = WorkflowService::new(db.clone());
        Self {
            db: db,
            document_service: Arc::new(document_service),
            form_service: Arc::new(form_service),
            workflow_service: Arc::new(workflow_service),
        }
    }
}
