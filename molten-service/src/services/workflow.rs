use crate::error::ServiceError;
use molten_core::WorkflowDefinition;
use molten_storage_seaorm::repo::WorkflowRepository;
use sea_orm::DatabaseConnection;

pub struct WorkflowService {
    db: DatabaseConnection,
}

impl WorkflowService {
    /// Create new instance of WorkflowService with database connection
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Saves the workflow to the database using SeaORM
    pub async fn save_workflow(
        &self,
        workflow: WorkflowDefinition,
    ) -> Result<WorkflowDefinition, ServiceError> {
        WorkflowRepository::save(&self.db, &workflow)
            .await
            .map_err(ServiceError::Internal)?;

        Ok(workflow)
    }

    /// Retrieves workflow by ID.
    pub async fn get_workflow(&self, id: &str) -> Result<WorkflowDefinition, ServiceError> {
        WorkflowRepository::find_by_id(&self.db, id)
            .await
            .map_err(ServiceError::Internal)?
            .ok_or_else(|| ServiceError::Internal(anyhow::anyhow!("Workflow not found")))
        // TODO: Implement specific NotFound error code
    }
}
