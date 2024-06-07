use utoipa::openapi::security::{HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        crate::controllers::auth::login::login,
        crate::controllers::auth::register::register,
        crate::controllers::auth::refresh::refresh,
    ),
    components(
        schemas(
            crate::controllers::auth::LoginRequest,
            crate::controllers::auth::LoginResponse,
            crate::controllers::auth::RegisterRequest,
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
