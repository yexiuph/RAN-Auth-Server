#![allow(non_snake_case,dead_code,unused_imports,unused_variables)]
mod utils;
mod routes;
mod models;

use crate::utils::ini::Config;
use crate::utils::database::{DatabaseState, connect_database};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::load();
    let database_url = config.get_database_url();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    
    env_logger::init();

    let db_pool = connect_database(database_url).await;

    println!("🟢 Server started successfully");
    println!("🟢 Authentication Version : {}", config.get_app_version());
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(DatabaseState { db_pool: db_pool.clone() }))
            .configure(routes::core_route)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(config.get_app_server())?
    .run()
    .await
}