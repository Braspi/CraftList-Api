//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa :: ToSchema,
)]
#[sea_orm(table_name = "players_graph")]
#[schema(title = "PlayersGraph")]
#[schema(as = crate::entities::players_graph::Model)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub server_id: i32,
    pub players_online: i32,
    pub date: Option<DateTime>,
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
}

impl Related<super::servers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Servers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
