use crate::error::ServiceError;
use molten_core::form::{FormBuilder, FormDefinition};
use molten_storage_seaorm::repo::FormRepository;
use sea_orm::DatabaseConnection;

pub struct FormService {
    db: DatabaseConnection,
}

impl FormService {
    /// Create new instance of FormService with database connection
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Creates a new form, validates it, and saves it to storage
    ///
    /// # Steps
    /// 1.
    pub async fn create_form(&self, form: FormDefinition) -> Result<FormDefinition, ServiceError> {
        FormRepository::save(&self.db, &form)
            .await
            .map_err(|e| ServiceError::Internal(e))?;

        Ok(form)
    }

    ///
    pub async fn get_form(&self, id: &str) -> Result<FormDefinition, ServiceError> {
        FormRepository::find_by_id(&self.db, id)
            .await
            .map_err(ServiceError::Internal)?
            .ok_or_else(|| ServiceError::Internal(anyhow::anyhow!("Form not found")))
        // TODO: Implement specific NotFound error code
    }
}
