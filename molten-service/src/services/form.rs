//! This module provides the service struct for Form entity operations.

use crate::error::ServiceError;
use molten_core::FormDefinition;
use molten_storage_seaorm::repo::FormRepository;
use molten_storage_seaorm::sea_orm::DatabaseConnection;

/// Service for managing form definitions.
///
/// This service handles the creation, retrieval, and persistence of `FormDefinition` objects.
pub struct FormService {
    db: DatabaseConnection,
}

impl FormService {
    /// Creates a new `FormService` instance.
    ///
    /// # Arguments
    /// * `db` - A `sea_orm::DatabaseConnection` used for database operations.
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Saves a given `FormDefinition` to the database.
    ///
    /// # Arguments
    /// * `form` - The `FormDefinition` to be saved.
    ///
    /// # Returns
    /// A `Result` which is `Ok(FormDefinition)` if the form was successfully saved,
    /// or `Err(ServiceError)` if a database error occurs.
    pub async fn save_form(&self, form: FormDefinition) -> Result<FormDefinition, ServiceError> {
        FormRepository::save(&self.db, &form)
            .await
            .map_err(ServiceError::Internal)?;

        Ok(form)
    }

    /// Retrieves a `FormDefinition` by its unique identifier.
    ///
    /// # Arguments
    /// * `id` - The unique ID of the form definition to retrieve.
    ///
    /// # Returns
    /// A `Result` which is `Ok(FormDefinition)` if the form is found, or `Err(ServiceError)`
    /// if the form is not found or a database error occurs.
    pub async fn get_form(&self, id: &str) -> Result<FormDefinition, ServiceError> {
        FormRepository::find_by_id(&self.db, id)
            .await
            .map_err(ServiceError::Internal)?
            .ok_or_else(|| ServiceError::Internal(anyhow::anyhow!("Form not found")))
        // TODO: Implement specific NotFound error code
    }
}
