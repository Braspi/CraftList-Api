use actix_web::{
    error::{ErrorBadRequest, ErrorConflict},
    web, HttpRequest, HttpResponse, Responder,
};
use craftping::ping;
use migration::{Alias, Expr, SimpleExpr};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QuerySelect, QueryTrait,
};
use serde_json::json;
use std::{collections::HashSet, sync::Arc};

use crate::{
    entities::{categories, server_categories, servers, servers_info, versions},
    error::AppError,
    utils::RequestUtils,
};

use super::ServerData;

fn vect_difference(v1: &[String], v2: &[String]) -> Vec<String> {
    let s1: HashSet<String> = v1.iter().cloned().collect();
    let s2: HashSet<String> = v2.iter().cloned().collect();
    (&s1 - &s2).iter().cloned().collect()
}

fn sub(version: &str) -> migration::SubQueryStatement {
    versions::Entity::find()
        .select_only()
        .column(versions::Column::Id)
        .filter(versions::Column::Name.eq(version))
        .into_query()
        .into_sub_query_statement()
}

#[utoipa::path(
    post,
    path = "/api/servers",
    tag = "Servers",
    params(
        ("Authentication" = String, Header, description = "JWT access token"),
    ),
    request_body(content = ServerData, description = "Server Data", content_type = "application/json"),
    responses(
        (status = 200, description = "Server object", body = Server),
        (status = 500, description = "Server error"),
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn add_server(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<ServerData>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    // Validate Categories
    let categories = categories::Entity::find()
        .filter(categories::Column::Name.is_in(data.categories.clone()))
        .all(db.get_ref().as_ref())
        .await?;

    let diff = vect_difference(
        &data.categories,
        &categories
            .iter()
            .map(|v| v.name.clone())
            .collect::<Vec<String>>(),
    );

    if !diff.is_empty() {
        return Err(
            ErrorBadRequest(format!("These categories are invalid: {}", diff.join(", "))).into(),
        );
    }

    // Validate Versions
    let versions: (Option<i32>, Option<i32>) = versions::Entity::find()
        .select_only()
        .column_as(
            SimpleExpr::SubQuery(None, Box::new(sub(&data.min_version))),
            "min_version",
        )
        .column_as(
            SimpleExpr::SubQuery(None, Box::new(sub(&data.max_version))),
            "max_version",
        )
        .group_by(Expr::col(Alias::new("min_version")))
        .group_by(Expr::col(Alias::new("min_version")))
        .into_tuple()
        .one(db.get_ref().as_ref())
        .await?
        .ok_or(AppError::Db(DbErr::RecordNotFound(
            "Could not find version data".to_owned(),
        )))?;

    if versions.0.is_none() || versions.1.is_none() {
        return Err(ErrorBadRequest("Version like this does not exist").into());
    }

    // Check if server already exists
    let servers: Vec<String> = servers::Entity::find()
        .all(db.get_ref().as_ref())
        .await?
        .iter()
        .map(|v| v.name.to_lowercase())
        .collect();

    if servers.contains(&data.name.to_lowercase()) {
        return Err(ErrorConflict("Server already exists").into());
    }

    if servers.len() >= 3 {
        return Err(ErrorBadRequest("You have reached limit of servers").into());
    }

    let new_server = servers::ActiveModel {
        name: Set(data.name.clone()),
        description: Set(data.description.clone()),
        is_premium: Set(false as i8),
        user_id: Set(req.get_user_id()?),
        ..Default::default()
    };
    let server = new_server.insert(db.get_ref().as_ref()).await?;

    let new_server_info = servers_info::ActiveModel {
        address: Set(data.address.clone()),
        server_id: Set(server.id),
        min_version: Set(versions.0.unwrap()),
        max_version: Set(versions.1.unwrap()),
        ..Default::default()
    };
    new_server_info.insert(db.get_ref().as_ref()).await?;

    let new_categories: Vec<server_categories::ActiveModel> = categories
        .iter()
        .map(|v| server_categories::ActiveModel {
            server_id: Set(server.id),
            category_id: Set(v.id),
        })
        .collect();

    server_categories::Entity::insert_many(new_categories)
        .exec(db.get_ref().as_ref())
        .await?;

    // let ping = ping(data.address.clone(), data.port).await;
    // ping.

    Ok(HttpResponse::Created().json(json!({"message": "Success"})))
}
