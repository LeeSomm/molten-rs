//! This module provides the SeaORM entity definition for Workflows.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a workflow definition entity stored in the database.
///
/// This struct defines the database model for a workflow, including its unique identifier,
/// name, and the full workflow graph (phases and transitions) as a JSON document.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workflows")]
pub struct Model {
    /// The unique identifier for the workflow definition (e.g., "standard_approval").
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    /// The name of the workflow.
    pub name: String,

    /// The entire `WorkflowDefinition` (Phases + Transitions) serialized as a JSON object.
    /// In Postgres, this is typically stored as a JSONB column to support indexing.
    #[sea_orm(column_type = "JsonBinary")]
    pub graph: Json,

    /// The timestamp when the workflow definition was created.
    pub created_at: DateTimeUtc,
    /// The timestamp when the workflow definition was last updated.
    pub updated_at: DateTimeUtc,
}

/// Defines relationships for the workflow entity.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    /// Establishes a one-to-many relationship with `Document` entities.
    #[sea_orm(has_many = "super::document::Entity")]
    Document,
}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Document.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
