#![allow(non_snake_case,dead_code,unused_imports,unused_variables)]
mod utils;
mod routes;
mod models;

use crate::utils::ini::{Config, ConfigState};
use crate::utils::database::{DatabaseState, connect_database};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Arc::new(Config::load());
    let database_url = config.get_database_url();
    let app_url = config.get_app_server();
    let config_state = actix_web::web::Data::new(ConfigState { config: config.clone() }); // Clone the Arc

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    
    env_logger::init();

    let db_pool = connect_database(database_url).await;

    println!("🟢 Server started successfully");
    println!("🟢 Authentication Version: {}", config_state.config.get_app_version());
    actix_web::HttpServer::new(move || {
        let config_clone = config.clone(); // Clone the Arc to be used in the closure
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(DatabaseState { db_pool: db_pool.clone() }))
            .app_data(config_state.clone())
            .configure(routes::core_route)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(app_url)?
    .run()
    .await
}