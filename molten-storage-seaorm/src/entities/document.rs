use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    /// UUID string
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,

    /// Foreign Key to forms table
    #[sea_orm(index)]
    pub form_id: String,

    /// Foreign Key to workflows table
    #[sea_orm(index)]
    pub workflow_id: String,

    /// Critical for fast workflow dashboards.
    #[sea_orm(index)]
    pub current_phase: String,

    /// The dynamic user data (key-value pairs).
    #[sea_orm(column_type = "JsonBinary")]
    pub data: Json,

    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::form::Entity",
        from = "Column::FormId",
        to = "super::form::Column::Id",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Form,

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
