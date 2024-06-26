use utoipa::openapi::security::{HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        crate::controllers::auth::login::login,
        crate::controllers::auth::register::register,
        crate::controllers::auth::refresh::refresh,

        // Categories
        crate::controllers::categories::add_category::add_category,
        crate::controllers::categories::list_categories::list_categories,
        crate::controllers::categories::update_category::update_category,
        crate::controllers::categories::remove_category::remove_category,

        // Versions
        crate::controllers::versions::add_version::add_version,
        crate::controllers::versions::list_versions::list_versions,
        crate::controllers::versions::update_version::update_version,
        crate::controllers::versions::remove_version::remove_version,

        // Servers
        crate::controllers::servers::list_servers::list_servers,
        crate::controllers::servers::get_server::get_server,
        crate::controllers::servers::get_user_servers::get_user_servers,
        crate::controllers::servers::add_server::add_server,
    ),
    components(
        // Auth
        schemas(
            crate::controllers::auth::LoginRequest,
            crate::controllers::auth::LoginResponse,
            crate::controllers::auth::RegisterRequest,
        ),

        // Categories
        schemas(
            crate::controllers::categories::Category,
            crate::controllers::categories::UpdateCategory,
            crate::controllers::categories::DeleteCategory,
        ),

        // Versions
        schemas(
            crate::controllers::versions::Version,
            crate::controllers::versions::UpdateVersion,
            crate::controllers::versions::DeleteVersion,
        ),

        // Servers
        schemas(
            crate::controllers::servers::Server,
            crate::controllers::servers::Category,
            crate::controllers::servers::ServerData,
        ),

        // Entities
        schemas(
            crate::entities::ads::Model,
            crate::entities::auth::Model,
            crate::entities::categories::Model,
            crate::entities::players_graph::Model,
            crate::entities::reviews::Model,
            crate::entities::server_categories::Model,
            crate::entities::servers::Model,
            crate::entities::servers_info::Model,
            crate::entities::users::Model,
            crate::entities::versions::Model,

            crate::entities::sea_orm_active_enums::Role,
        ),

        // Errors
        schemas(
            crate::error::AppError,
        ),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Craftlist", description = "Craftlist documentation")
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "Authorization",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
