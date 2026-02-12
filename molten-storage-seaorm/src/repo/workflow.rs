//! Repository implementation for interacting with Workflow entities in the database.

use crate::entities::workflow;
use crate::entities::workflow::Entity as WorkflowEntity;
use anyhow::Result;
use molten_core::workflow::WorkflowDefinition;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

/// Repository for `WorkflowDefinition` entities, providing CRUD operations.
///
/// This struct acts as a data access layer for workflow definitions, abstracting the underlying
/// SeaORM implementation.
pub struct WorkflowRepository;

impl WorkflowRepository {
    /// Saves a `WorkflowDefinition` to the database.
    ///
    /// If a workflow with the same ID already exists, it will be updated.
    ///
    /// # Arguments
    /// * `db` - A reference to the `DatabaseConnection`.
    /// * `def` - A reference to the `WorkflowDefinition` domain model to be saved.
    ///
    /// # Returns
    /// `Result<()>` indicating success or failure.
    pub async fn save(db: &DatabaseConnection, def: &WorkflowDefinition) -> Result<()> {
        let active_model = workflow::ActiveModel {
            id: Set(def.id().to_string()),
            name: Set(def.name().to_string()),
            graph: Set(serde_json::to_value(def)?),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        workflow::Entity::insert(active_model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(workflow::Column::Id)
                    .update_columns([
                        workflow::Column::Name,
                        workflow::Column::Graph,
                        workflow::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    /// Retrieves a `WorkflowDefinition` by its ID.
    ///
    /// # Arguments
    /// * `db` - A reference to the `DatabaseConnection`.
    /// * `id` - The ID of the workflow definition to retrieve.
    ///
    /// # Returns
    /// `Result<Option<WorkflowDefinition>>` where `Some(WorkflowDefinition)` is returned if found,
    /// `None` if not found, or an `Err` if a database error occurs.
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<Option<WorkflowDefinition>> {
        let model = WorkflowEntity::find_by_id(id).one(db).await?;

        match model {
            Some(m) => {
                let def: WorkflowDefinition = serde_json::from_value(m.graph)?;
                Ok(Some(def))
            }
            None => Ok(None),
        }
    }
}
