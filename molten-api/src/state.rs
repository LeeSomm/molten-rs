//! Defines the shared application state for the Molten API.
//!
//! This module provides the `AppState` struct, which holds common resources
//! such as the database connection and service clients, making them
//! accessible to all request handlers.
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
    /// Creates a new instance of `AppState`, initializing all shared services
    /// with the provided database connection.
    ///
    /// # Arguments
    /// * `db` - A `DatabaseConnection` to be used by the services.
    ///
    /// # Returns
    /// A new `AppState` instance.
    pub fn new(db: DatabaseConnection) -> Self {
        let document_service = DocumentService::new(db.clone());
        let form_service = FormService::new(db.clone());
        let workflow_service = WorkflowService::new(db.clone());
        Self {
            db,
            document_service: Arc::new(document_service),
            form_service: Arc::new(form_service),
            workflow_service: Arc::new(workflow_service),
        }
    }
}
