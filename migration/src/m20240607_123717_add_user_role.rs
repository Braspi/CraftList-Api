use sea_orm_migration::prelude::*;

use crate::m20240531_134809_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("role"))
                            .enumeration(
                                Alias::new("role"),
                                vec![Alias::new("Admin"), Alias::new("User")],
                            )
                            .default("User")
                            .not_null()
                            .extra("AFTER password"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Alias::new("role"))
                    .to_owned(),
            )
            .await
    }
}
