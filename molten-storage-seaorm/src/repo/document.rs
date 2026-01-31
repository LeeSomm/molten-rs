use crate::entities::document;
use crate::entities::document::Entity as DocumentEntity;
use anyhow::Result;
use molten_core::document::Document;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use serde_json::Value;
use std::collections::HashMap; // Using anyhow for simplified error handling in storage layer

pub struct DocumentRepository;

impl DocumentRepository {
    /// Inserts a new document into the database.
    pub async fn create(db: &DatabaseConnection, doc: &Document) -> Result<()> {
        // Convert Domain Model -> ActiveModel
        let active_model = document::ActiveModel {
            id: Set(doc.id.clone()),
            form_id: Set(doc.form_id.clone()),
            workflow_id: Set(doc.workflow_id.clone()),
            current_phase: Set(doc.current_phase.clone()),
            // Serialize the HashMap into a JSON Value
            data: Set(serde_json::to_value(&doc.data)?),
            created_at: Set(doc.created_at.into()),
            updated_at: Set(doc.updated_at.into()),
        };

        active_model.insert(db).await?;
        Ok(())
    }

    /// Retrieves a document by ID and converts it back to the Domain Model.
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
                    created_at: m.created_at.into(), // Assumes SeaORM DateTimeUtc -> Chrono conversion
                    updated_at: m.updated_at.into(),
                }))
            }
            None => Ok(None),
        }
    }

    /// Updates an existing document's data and phase.
    pub async fn update(db: &DatabaseConnection, doc: &Document) -> Result<()> {
        let active_model = document::ActiveModel {
            id: Set(doc.id.clone()), // Primary key determines which row to update
            current_phase: Set(doc.current_phase.clone()),
            data: Set(serde_json::to_value(&doc.data)?),
            updated_at: Set(chrono::Utc::now().into()),
            ..Default::default() // Don't touch other fields (form_id, created_at)
        };

        active_model.update(db).await?;
        Ok(())
    }

    /// Finds all documents currently in a specific phase (e.g., "Review").
    /// This utilizes our indexed `current_phase` column for speed.
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
                created_at: m.created_at.into(),
                updated_at: m.updated_at.into(),
            });
        }

        Ok(docs)
    }
}
