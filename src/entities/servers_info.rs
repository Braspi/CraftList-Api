//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa :: ToSchema,
)]
#[sea_orm(table_name = "servers_info")]
#[schema(title = "changeThis")]
# [schema (as = crate :: entities :: changeThis :: Model)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub server_id: i32,
    pub address: String,
    pub min_version: i32,
    pub max_version: i32,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::servers::Entity",
        from = "Column::ServerId",
        to = "super::servers::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Servers,
    #[sea_orm(
        belongs_to = "super::versions::Entity",
        from = "Column::MaxVersion",
        to = "super::versions::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Versions2,
    #[sea_orm(
        belongs_to = "super::versions::Entity",
        from = "Column::MinVersion",
        to = "super::versions::Column::Id",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Versions1,
}

impl Related<super::servers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Servers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
