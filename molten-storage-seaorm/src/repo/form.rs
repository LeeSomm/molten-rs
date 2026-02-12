//! Repository implementation for interacting with Form entities in the database.

use crate::entities::form;
use crate::entities::form::Entity as FormEntity;
use anyhow::Result;
use molten_core::form::FormDefinition;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

/// Repository for `FormDefinition` entities, providing CRUD operations.
///
/// This struct acts as a data access layer for form definitions, abstracting the underlying
/// SeaORM implementation.
pub struct FormRepository;

impl FormRepository {
    /// Saves a `FormDefinition` to the database.
    ///
    /// If a form with the same ID already exists, it will be updated.
    ///
    /// # Arguments
    /// * `db` - A reference to the `DatabaseConnection`.
    /// * `def` - A reference to the `FormDefinition` domain model to be saved.
    ///
    /// # Returns
    /// `Result<()>` indicating success or failure.
    pub async fn save(db: &DatabaseConnection, def: &FormDefinition) -> Result<()> {
        // We store the *entire* definition as JSON, but also pull out
        // name/version for SQL columns.
        let active_model = form::ActiveModel {
            id: Set(def.id().to_string()),
            name: Set(def.name().to_string()),
            version: Set(def.version() as i32),
            schema: Set(serde_json::to_value(def)?),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
        };

        // TODO: Separate 'insert' vs 'update' logic here.
        // For now, we'll assume simple insertion of new versions.
        form::Entity::insert(active_model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(form::Column::Id)
                    .update_columns([
                        form::Column::Name,
                        form::Column::Version,
                        form::Column::Schema,
                        form::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    /// Retrieves a `FormDefinition` by its ID.
    ///
    /// # Arguments
    /// * `db` - A reference to the `DatabaseConnection`.
    /// * `id` - The ID of the form definition to retrieve.
    ///
    /// # Returns
    /// `Result<Option<FormDefinition>>` where `Some(FormDefinition)` is returned if found,
    /// `None` if not found, or an `Err` if a database error occurs.
    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> Result<Option<FormDefinition>> {
        let model = FormEntity::find_by_id(id).one(db).await?;

        match model {
            Some(m) => {
                // We deserializing primarily from the JSON column
                let def: FormDefinition = serde_json::from_value(m.schema)?;
                Ok(Some(def))
            }
            None => Ok(None),
        }
    }
}
