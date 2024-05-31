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
                    .table(Ads::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ads::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ads::Image).string().not_null())
                    .col(ColumnDef::new(Ads::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Ads::CreatedAt)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(ColumnDef::new(Ads::ExpiresAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_Ads_Users")
                            .from(Ads::Table, Ads::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ads::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Ads {
    Table,
    Id,
    Image,
    UserId,
    CreatedAt,
    ExpiresAt,
}
