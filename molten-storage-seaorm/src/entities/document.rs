//! This module provides the SeaORM entity definition for Documents.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a document entity stored in the database.
///
/// This struct defines the database model for a document, including its unique identifier,
/// associated form and workflow IDs, current phase, and dynamic data.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    /// The unique identifier for the document. This is a UUID string.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    /// The foreign key linking to the associated form definition.
    #[sea_orm(index)]
    pub form_id: String,

    /// The foreign key linking to the associated workflow definition.
    #[sea_orm(index)]
    pub workflow_id: String,

    /// The current phase of the document within its workflow.
    /// Critical for fast workflow dashboards and querying.
    #[sea_orm(index)]
    pub current_phase: String,

    /// The dynamic user-defined data associated with the document, stored as JSON.
    #[sea_orm(column_type = "JsonBinary")]
    pub data: Json,

    /// The timestamp when the document was created.
    pub created_at: DateTimeUtc,
    /// The timestamp when the document was last updated.
    pub updated_at: DateTimeUtc,
}

/// Defines relationships for the document entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Establishes a many-to-one relationship with the `Form` entity.
    #[sea_orm(
        belongs_to = "super::form::Entity",
        from = "Column::FormId",
        to = "super::form::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Form,

    /// Establishes a many-to-one relationship with the `Workflow` entity.
    #[sea_orm(
        belongs_to = "super::workflow::Entity",
        from = "Column::WorkflowId",
        to = "super::workflow::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Workflow,
}

impl Related<super::form::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Form.def()
    }
}

impl Related<super::workflow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workflow.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
