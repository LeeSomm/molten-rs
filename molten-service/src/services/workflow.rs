//! This module provides the service struct for Workflow entity operations.
use crate::error::ServiceError;
use molten_core::WorkflowDefinition;
use molten_storage_seaorm::repo::WorkflowRepository;
use sea_orm::DatabaseConnection;

/// Service for managing workflow definitions.
///
/// This service handles the creation, retrieval, and persistence of `WorkflowDefinition` objects.
pub struct WorkflowService {
    db: DatabaseConnection,
}

impl WorkflowService {
    /// Creates a new `WorkflowService` instance.
    ///
    /// # Arguments
    /// * `db` - A `sea_orm::DatabaseConnection` used for database operations.
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Saves a given `WorkflowDefinition` to the database.
    ///
    /// # Arguments
    /// * `workflow` - The `WorkflowDefinition` to be saved.
    ///
    /// # Returns
    /// A `Result` which is `Ok(WorkflowDefinition)` if the workflow was successfully saved,
    /// or `Err(ServiceError)` if a database error occurs.
    pub async fn save_workflow(
        &self,
        workflow: WorkflowDefinition,
    ) -> Result<WorkflowDefinition, ServiceError> {
        WorkflowRepository::save(&self.db, &workflow)
            .await
            .map_err(ServiceError::Internal)?;

        Ok(workflow)
    }

    /// Retrieves a `WorkflowDefinition` by its unique identifier.
    ///
    /// # Arguments
    /// * `id` - The unique ID of the workflow definition to retrieve.
    ///
    /// # Returns
    /// A `Result` which is `Ok(WorkflowDefinition)` if the workflow is found, or `Err(ServiceError)`
    /// if the workflow is not found or a database error occurs.
    pub async fn get_workflow(&self, id: &str) -> Result<WorkflowDefinition, ServiceError> {
        WorkflowRepository::find_by_id(&self.db, id)
            .await
            .map_err(ServiceError::Internal)?
            .ok_or_else(|| ServiceError::Internal(anyhow::anyhow!("Workflow not found")))
        // TODO: Implement specific NotFound error code
    }
}
