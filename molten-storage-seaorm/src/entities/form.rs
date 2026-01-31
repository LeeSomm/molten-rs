use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "forms")]
pub struct Model {
    /// The unique form ID (e.g., "incident_report").
    /// Corresponds to `FormDefinition.id`.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    /// Duplicated from schema for fast searching.
    pub name: String,

    /// Duplicated from schema for fast version checks.
    pub version: i32,

    /// The heavy lifting: The entire FormDefinition struct serialized as JSON.
    /// In Postgres, this should be a JSONB column for indexing support.
    #[sea_orm(column_type = "JsonBinary")]
    pub schema: Json,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::document::Entity")]
    Document,
}

impl Related<super::document::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Document.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
