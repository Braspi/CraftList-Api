use sea_orm_migration::prelude::*;

use crate::m20240531_134809_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("email")
                    .table(Users::Table)
                    .col(Users::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("username")
                    .table(Users::Table)
                    .col(Users::Username)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().table(Users::Table).name("email").to_owned())
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .table(Users::Table)
                    .name("username")
                    .to_owned(),
            )
            .await
    }
}
