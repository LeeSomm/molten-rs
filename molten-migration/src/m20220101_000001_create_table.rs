use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create Forms Table
        manager
            .create_table(
                Table::create()
                    .table(Forms::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Forms::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Forms::Name).string().not_null())
                    .col(ColumnDef::new(Forms::Version).integer().not_null())
                    // Store the full definition as JSONB
                    .col(ColumnDef::new(Forms::Schema).json_binary().not_null())
                    .col(
                        ColumnDef::new(Forms::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Forms::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Create Workflows Table
        manager
            .create_table(
                Table::create()
                    .table(Workflows::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Workflows::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Workflows::Name).string().not_null())
                    // Store the graph as JSONB
                    .col(ColumnDef::new(Workflows::Graph).json_binary().not_null())
                    .col(
                        ColumnDef::new(Workflows::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Workflows::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 3. Create Documents Table
        manager
            .create_table(
                Table::create()
                    .table(Documents::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Documents::Id).string().not_null().primary_key())
                    // Foreign Keys are loose for now to allow soft-deletes of config, 
                    // but we index them for speed.
                    .col(ColumnDef::new(Documents::FormId).string().not_null())
                    .col(ColumnDef::new(Documents::WorkflowId).string().not_null())
                    
                    // Promoted column for fast status checks
                    .col(ColumnDef::new(Documents::CurrentPhase).string().not_null())
                    
                    // The dynamic data payload
                    .col(ColumnDef::new(Documents::Data).json_binary().not_null())
                    
                    .col(
                        ColumnDef::new(Documents::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Documents::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. Create Indexes for Documents Table
        manager
            .create_index(
                Index::create()
                    .name("idx_documents_form_id")
                    .table(Documents::Table)
                    .col(Documents::FormId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_documents_current_phase")
                    .table(Documents::Table)
                    .col(Documents::CurrentPhase)
                    .to_owned(),
            )
            .await

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Documents::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Workflows::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Forms::Table).to_owned()).await
    }
}

/// Helper Enums to avoid using string literals for table/column names.
/// This ensures typos are caught at compile time.
#[derive(Iden)]
enum Forms {
    Table,
    Id,
    Name,
    Version,
    Schema,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Workflows {
    Table,
    Id,
    Name,
    Graph,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Documents {
    Table,
    Id,
    FormId,
    WorkflowId,
    CurrentPhase,
    Data,
    CreatedAt,
    UpdatedAt,
}