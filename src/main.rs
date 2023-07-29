mod utils;
mod routes;

use actix_web::{HttpServer, App, web};
use actix_web::middleware::Logger;

use crate::utils::ini::Config;
use crate::utils::database::{AppState, connect_database};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Config::load();
    let database_url = config.database_url();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    
    env_logger::init();

    let db_pool = connect_database(database_url).await;

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db_pool: db_pool.clone() }))
            .configure(routes::core_route) // <-- This line adds the routes from the routes module
            .wrap(Logger::default())
    })
    .bind(config.app_server())?
    .run()
    .await
}