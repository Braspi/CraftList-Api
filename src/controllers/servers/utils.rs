use migration::{Alias, Expr};
use sea_orm::{EntityTrait, QuerySelect, Select};

use crate::entities::{categories, server_categories, servers, servers_info, versions};

pub fn get_server() -> Select<servers::Entity> {
    servers::Entity::find()
        .join(
            sea_orm::JoinType::InnerJoin,
            servers::Entity::belongs_to(servers_info::Entity)
                .from(servers::Column::Id)
                .to(servers_info::Column::ServerId)
                .into(),
        )
        .join_as(
            sea_orm::JoinType::InnerJoin,
            servers_info::Entity::belongs_to(versions::Entity)
                .from(servers_info::Column::MinVersion)
                .to(versions::Column::Id)
                .into(),
            Alias::new("v1"),
        )
        .join_as(
            sea_orm::JoinType::InnerJoin,
            servers_info::Entity::belongs_to(versions::Entity)
                .from(servers_info::Column::MaxVersion)
                .to(versions::Column::Id)
                .into(),
            Alias::new("v2"),
        )
        .join(
            sea_orm::JoinType::InnerJoin,
            servers_info::Entity::belongs_to(server_categories::Entity)
                .from(servers_info::Column::ServerId)
                .to(server_categories::Column::ServerId)
                .into(),
        )
        .join(
            sea_orm::JoinType::InnerJoin,
            server_categories::Entity::belongs_to(categories::Entity)
                .from(server_categories::Column::CategoryId)
                .to(categories::Column::Id)
                .into(),
        )
        .group_by(servers_info::Column::Id)
        .group_by(servers::Column::Name)
        .group_by(servers_info::Column::Address)
        .group_by(Expr::col((Alias::new("v1"), versions::Column::Name)))
        .group_by(Expr::col((Alias::new("v2"), versions::Column::Name)))
        .select_only()
        .column(servers::Column::Id)
        .column(servers::Column::Name)
        .column(servers::Column::Description)
        .column(servers::Column::UserId)
        .column(servers::Column::IsPremium)
        .column(servers::Column::CreatedAt)
        .column(servers_info::Column::Address)
        .column_as(
            Expr::col((Alias::new("v1"), versions::Column::Name)),
            "min_version",
        )
        .column_as(
            Expr::col((Alias::new("v2"), versions::Column::Name)),
            "max_version",
        )
        .expr_as(
            Expr::cust("JSON_ARRAYAGG(JSON_OBJECT('id', categories.id, 'name', categories.name))"),
            "categories",
        )
        .to_owned()
}
