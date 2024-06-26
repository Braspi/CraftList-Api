pub use sea_orm_migration::prelude::*;

mod m20240531_134809_create_users_table;
mod m20240531_140055_create_auth_table;
mod m20240531_140153_create_servers_table;
mod m20240531_140157_create_versions_table;
mod m20240531_140207_create_categories_table;
mod m20240531_140213_create_servers_info_table;
mod m20240531_140220_create_server_categories_table;
mod m20240531_140238_create_ads_table;
mod m20240531_140243_create_players_graph_table;
mod m20240531_140339_create_reviews_table;
mod m20240604_185157_make_email_unique;
mod m20240607_123717_add_user_role;
mod m20240612_180337_add_version_protocol;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240531_134809_create_users_table::Migration),
            Box::new(m20240531_140055_create_auth_table::Migration),
            Box::new(m20240531_140153_create_servers_table::Migration),
            Box::new(m20240531_140157_create_versions_table::Migration),
            Box::new(m20240531_140207_create_categories_table::Migration),
            Box::new(m20240531_140213_create_servers_info_table::Migration),
            Box::new(m20240531_140220_create_server_categories_table::Migration),
            Box::new(m20240531_140238_create_ads_table::Migration),
            Box::new(m20240531_140243_create_players_graph_table::Migration),
            Box::new(m20240531_140339_create_reviews_table::Migration),
            Box::new(m20240604_185157_make_email_unique::Migration),
            Box::new(m20240607_123717_add_user_role::Migration),
            Box::new(m20240612_180337_add_version_protocol::Migration),
        ]
    }
}
