use std::{fs::File, io::Read};

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;

mod entities;

#[derive(Deserialize)]
struct Config {
    addr: String,
    port: u16,
    threads: usize,
    database_table: String,
    database_url: String,
    log: u32,
}

fn load_config() -> Config {
    let mut file = File::open("config.json").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    serde_json::from_str(&str).unwrap()
}

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config: Config = load_config();

    let mut opt = ConnectOptions::new(format!("{}/{}", config.database_url, config.database_table));
    opt.sqlx_logging_level(log::LevelFilter::Debug);
    let conn = Database::connect(opt).await.unwrap();

    Migrator::up(&conn, None).await.unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default().log_target("CraftList"))
    })
    .workers(config.threads)
    .bind((config.addr, config.port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
