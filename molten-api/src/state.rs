use molten_service::{
    DocumentService,
    form_service::{self, FormService},
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// The shared state accessible by all request handlers.
/// We wrap it in Arc for cheap cloning across threads.
#[derive(Clone)]
pub struct AppState {
    pub document_service: Arc<DocumentService>,
    pub form_service: Arc<FormService>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        let document_service = DocumentService::new(db.clone());
        let form_service = FormService::new(db.clone());
        Self {
            document_service: Arc::new(document_service),
            form_service: Arc::new(form_service),
        }
    }
}
