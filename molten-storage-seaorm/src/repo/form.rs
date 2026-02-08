use crate::entities::form;
use crate::entities::form::Entity as FormEntity;
use anyhow::Result;
use molten_core::form::FormDefinition;
use sea_orm::{DatabaseConnection, EntityTrait, Set};

pub struct FormRepository;

impl FormRepository {
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
