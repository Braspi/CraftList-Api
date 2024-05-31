//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, utoipa :: ToSchema,
)]
#[sea_orm(table_name = "categories")]
#[schema(title = "changeThis")]
# [schema (as = crate :: entities :: changeThis :: Model)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::server_categories::Entity")]
    ServerCategories,
}

impl Related<super::server_categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ServerCategories.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
