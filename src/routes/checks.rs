use actix_web::{get, HttpResponse , Responder};
use crate::AppState;
use serde_json::json;

#[get("/api/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "success", "message": "OK"}))
}

#[get("/api/databasechecker")]
pub async fn database_checker_handler(database: actix_web::web::Data<AppState>) -> impl Responder {
    match database.check_connection().await {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success", "message": "OK"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Database connection failed: {}", err)}))
    }
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(database_checker_handler);
    cfg.service(health_checker_handler);
}