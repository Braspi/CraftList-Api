use sea_orm_migration::prelude::*;

use crate::m20240531_134809_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Servers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Servers::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Servers::Name).string().not_null())
                    .col(ColumnDef::new(Servers::Description).string().not_null())
                    .col(ColumnDef::new(Servers::UserId).integer().not_null())
                    .col(ColumnDef::new(Servers::IsPremium).boolean().not_null())
                    .col(
                        ColumnDef::new(Servers::CreatedAt)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_Servers_Users")
                            .from(Servers::Table, Servers::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Servers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Servers {
    Table,
    Id,
    Name,
    Description,
    UserId,
    IsPremium,
    CreatedAt,
}
