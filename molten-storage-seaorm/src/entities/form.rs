//! This module provides the SeaORM entity definition for Forms.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a form definition entity stored in the database.
///
/// This struct defines the database model for a form, including its unique identifier,
/// name, version, and the full schema as a JSON document.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "forms")]
pub struct Model {
    /// The unique identifier for the form definition (e.g., "incident_report").
    /// Corresponds to `FormDefinition.id`.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    /// The name of the form, duplicated from the schema for fast searching.
    pub name: String,

    /// The version of the form definition, duplicated from the schema for fast version checks.
    pub version: i32,

    /// The complete `FormDefinition` struct serialized as a JSON object.
    /// In Postgres, this is typically stored as a JSONB column to support indexing.
    #[sea_orm(column_type = "JsonBinary")]
    pub schema: Json,

    /// The timestamp when the form definition was created.
    pub created_at: DateTimeUtc,
    /// The timestamp when the form definition was last updated.
    pub updated_at: DateTimeUtc,
}

/// Defines relationships for the form entity.
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
