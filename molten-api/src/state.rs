use molten_service::DocumentService;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// The shared state accessible by all request handlers.
/// We wrap it in Arc for cheap cloning across threads.
#[derive(Clone)]
pub struct AppState {
    pub service: Arc<DocumentService>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        let service = DocumentService::new(db);
        Self {
            service: Arc::new(service),
        }
    }
}
