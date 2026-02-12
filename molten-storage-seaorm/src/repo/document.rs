//! Repository implementation for interacting with Document entities in the database.

use crate::entities::document;
use crate::entities::document::Entity as DocumentEntity;
use anyhow::Result;
use molten_core::document::Document;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::Value;
use std::collections::HashMap; // Using anyhow for simplified error handling in storage layer

/// Repository for `Document` entities, providing CRUD operations and query capabilities.
///
/// This struct acts as a data access layer for documents, abstracting the underlying
/// SeaORM implementation.
pub struct DocumentRepository;

impl DocumentRepository {
    /// Inserts a new `Document` domain model into the database.
    ///
    /// # Arguments
    /// * `db` - A reference to the `DatabaseConnection`.
    /// * `doc` - A reference to the `Document` domain model to be created.
    ///
    /// # Returns
    /// `Result<()>` indicating success or failure.
    pub async fn create(db: &DatabaseConnection, doc: &Document) -> Result<()> {
        // Convert Domain Model -> ActiveModel
        let active_model = document::ActiveModel {
            id: Set(doc.id.clone()),
            form_id: Set(doc.form_id.clone()),
            workflow_id: Set(doc.workflow_id.clone()),
            current_phase: Set(doc.current_phase.clone()),
            // Serialize the HashMap into a JSON Value
            data: Set(serde_json::to_value(&doc.data)?),
            created_at: Set(doc.created_at),
            updated_at: Set(doc.updated_at),
        };

        active_model.insert(db).await?;
        Ok(())
    }

    /// Retrieves a document by its ID and converts it to a `Document` domain model.
    ///
    /// # Arguments
    /// * `db` - A reference to the `DatabaseConnection`.
    /// * `id` - The ID of the document to retrieve.
    ///
    /// # Returns
    /// `Result<Option<Document>>` where `Some(Document)` is returned if found,
    /// `None` if not found, or an `Err` if a database error occurs.
    pub async fn find_by_id(db: &DatabaseConnection, id: &str) -> Result<Option<Document>> {
        let model = DocumentEntity::find_by_id(id).one(db).await?;

        match model {
            Some(m) => {
                // Convert DB Model -> Domain Model
                let data_map: HashMap<String, Value> = serde_json::from_value(m.data)?;

                Ok(Some(Document {
                    id: m.id,
                    form_id: m.form_id,
                    workflow_id: m.workflow_id,
                    current_phase: m.current_phase,
                    data: data_map,
                    created_at: m.created_at,
                    updated_at: m.updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    /// Updates an existing document's `current_phase` and `data` fields in the database.
    ///
    /// # Arguments
    /// * `db` - A reference to the `DatabaseConnection`.
    /// * `doc` - A reference to the `Document` domain model containing the updated fields.
    ///
    /// # Returns
    /// `Result<()>` indicating success or failure.
    pub async fn update(db: &DatabaseConnection, doc: &Document) -> Result<()> {
        let active_model = document::ActiveModel {
            id: Set(doc.id.clone()), // Primary key determines which row to update
            current_phase: Set(doc.current_phase.clone()),
            data: Set(serde_json::to_value(&doc.data)?),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default() // Don't touch other fields (form_id, created_at)
        };

        active_model.update(db).await?;
        Ok(())
    }

    /// Finds all documents that are currently in a specific phase.
    ///
    /// This method leverages the indexed `current_phase` column for efficient querying.
    ///
    /// # Arguments
    /// * `db` - A reference to the `DatabaseConnection`.
    /// * `phase` - The name of the phase to filter documents by.
    ///
    /// # Returns
    /// `Result<Vec<Document>>` a vector of `Document` domain models, or an `Err` if a database error occurs.
    pub async fn find_by_phase(db: &DatabaseConnection, phase: &str) -> Result<Vec<Document>> {
        let models = DocumentEntity::find()
            .filter(document::Column::CurrentPhase.eq(phase))
            .all(db)
            .await?;

        // Map all results back to Domain Models
        let mut docs = Vec::new();
        for m in models {
            let data_map: HashMap<String, Value> = serde_json::from_value(m.data)?;
            docs.push(Document {
                id: m.id,
                form_id: m.form_id,
                workflow_id: m.workflow_id,
                current_phase: m.current_phase,
                data: data_map,
                created_at: m.created_at,
                updated_at: m.updated_at,
            });
        }

        Ok(docs)
    }
}
