use sea_orm_migration::prelude::*;

use crate::m20240531_140157_create_versions_table::Versions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Versions::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("protocol"))
                            .integer()
                            .not_null()
                            .default(47)
                            .extra("AFTER name"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Versions::Table)
                    .drop_column(Alias::new("protocol"))
                    .to_owned(),
            )
            .await
    }
}
