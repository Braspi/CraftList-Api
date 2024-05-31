use sea_orm_migration::prelude::*;

use crate::m20240531_140153_create_servers_table::Servers;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlayersGraph::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PlayersGraph::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PlayersGraph::ServerId).integer().not_null())
                    .col(
                        ColumnDef::new(PlayersGraph::PlayersOnline)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PlayersGraph::Date)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_PlayersGraph_Servers")
                            .from(PlayersGraph::Table, PlayersGraph::ServerId)
                            .to(Servers::Table, Servers::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlayersGraph::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PlayersGraph {
    Table,
    Id,
    ServerId,
    PlayersOnline,
    Date,
}
