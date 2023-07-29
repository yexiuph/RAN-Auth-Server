mod utils;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde_json::json;


use crate::utils::ini::Config;
use crate::utils::database::{AppState, connect_database};

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success", "message": "OK"}))
}

#[get("/api/databasechecker")]
async fn database_checker_handler(database: web::Data<AppState>) -> impl Responder {
    match database.check_connection().await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "OK"})),
        Err(err) => {
            println!("{}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Database connection failed: {}", err)}))
        },
    }
}

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
            .service(health_checker_handler)
            .service(database_checker_handler)
            .wrap(Logger::default())
    })
    .bind(config.app_server())?
    .run()
    .await
}