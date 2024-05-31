use sea_orm_migration::prelude::*;

use crate::{
    m20240531_134809_create_users_table::Users, m20240531_140153_create_servers_table::Servers,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reviews::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reviews::UserId).integer().not_null())
                    .col(ColumnDef::new(Reviews::ServerId).integer().not_null())
                    .col(ColumnDef::new(Reviews::Description).string().not_null())
                    .col(ColumnDef::new(Reviews::Stars).integer().not_null())
                    .col(
                        ColumnDef::new(Reviews::CreatedAt)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_Reviews_Users")
                            .from(Reviews::Table, Reviews::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_Reviews_Servers")
                            .from(Reviews::Table, Reviews::ServerId)
                            .to(Servers::Table, Servers::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reviews::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    UserId,
    ServerId,
    Description,
    Stars,
    CreatedAt,
}
