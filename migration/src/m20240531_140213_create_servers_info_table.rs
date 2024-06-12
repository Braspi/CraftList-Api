use sea_orm_migration::prelude::*;

use crate::{
    m20240531_140153_create_servers_table::Servers,
    m20240531_140157_create_versions_table::Versions,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServersInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServersInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServersInfo::ServerId).integer().not_null())
                    .col(ColumnDef::new(ServersInfo::Address).string().not_null())
                    .col(ColumnDef::new(ServersInfo::MinVersion).integer().not_null())
                    .col(ColumnDef::new(ServersInfo::MaxVersion).integer().not_null())
                    .col(
                        ColumnDef::new(ServersInfo::CreatedAt)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(ServersInfo::UpdatedAt)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ServersInfo_Servers")
                            .from(ServersInfo::Table, ServersInfo::ServerId)
                            .to(Servers::Table, Servers::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ServersInfo_MinVersion_Version_Id")
                            .from(ServersInfo::Table, ServersInfo::MinVersion)
                            .to(Versions::Table, Versions::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_ServersInfo_MaxVersion_Version_Id")
                            .from(ServersInfo::Table, ServersInfo::MaxVersion)
                            .to(Versions::Table, Versions::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServersInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ServersInfo {
    Table,
    Id,
    ServerId,
    Address,
    MinVersion,
    MaxVersion,
    CreatedAt,
    UpdatedAt,
}
