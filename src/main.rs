#![allow(non_snake_case, dead_code, unused_imports, unused_variables)]
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

use crate::utils::database::{connect_database, DatabaseState};
use crate::utils::ini::{Config, ConfigState};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Arc::new(Config::load());
    let database_url = config.get_database_url();
    let app_url = config.get_app_server();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();

    let db_pool = connect_database(database_url).await;

    println!("🟢 RAN API Server started successfully");
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(DatabaseState {
                db_pool: db_pool.clone(),
            }))
            .app_data(actix_web::web::Data::new(ConfigState {
                config: config.clone(),
            }))
            .configure(|cfg| routes::core_route(cfg))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(app_url)?
    .run()
    .await
}
