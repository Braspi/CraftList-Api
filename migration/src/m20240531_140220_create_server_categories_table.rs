use sea_orm_migration::prelude::*;

use crate::{
    m20240531_140207_create_categories_table::Categories,
    m20240531_140213_create_servers_info_table::ServersInfo,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServerCategories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServerCategories::ServerId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServerCategories::CategoryId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ServerCategories_Servers")
                            .from(ServerCategories::Table, ServerCategories::ServerId)
                            .to(ServersInfo::Table, ServersInfo::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ServerCategories_Categories")
                            .from(ServerCategories::Table, ServerCategories::CategoryId)
                            .to(Categories::Table, Categories::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServerCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ServerCategories {
    Table,
    ServerId,
    CategoryId,
}
