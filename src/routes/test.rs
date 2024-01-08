use serde_json::json;

#[actix_web::get("/test")]
pub async fn test_handler() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().json(json!({"status": "success", "message": "OK"}))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(test_handler);
}
