use actix_web::{web, get, post, HttpResponse, HttpRequest, Responder};
use crate::{DatabaseState, ConfigState};
use serde_json::json;

#[post("/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success", "message": "OK"}))
}

#[post("/databasechecker")]
pub async fn database_checker_handler(database: web::Data<DatabaseState>) -> impl Responder {
    match database.check_connection().await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "OK"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Database connection failed: {}", err)}))
    }
}

#[post("/version")]
pub async fn version_check(configData: web::Data<ConfigState>) -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success", "version":  configData.config.get_app_version()}))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
        .service(database_checker_handler)
        .service(version_check)
        .service(health_checker_handler);
}