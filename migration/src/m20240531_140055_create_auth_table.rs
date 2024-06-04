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
                    .table(Auth::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Auth::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Auth::Token).string().not_null())
                    .col(ColumnDef::new(Auth::UserId).integer().not_null())
                    .col(
                        ColumnDef::new(Auth::CreatedAt)
                            .date_time()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(ColumnDef::new(Auth::ExpiresAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_Auth_Users")
                            .from(Auth::Table, Auth::UserId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Auth::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Auth {
    Table,
    Id,
    Token,
    UserId,
    CreatedAt,
    ExpiresAt,
}
