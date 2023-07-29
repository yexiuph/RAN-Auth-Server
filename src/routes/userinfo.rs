use actix_web::{get, HttpResponse , Responder};
use crate::{
    models::userinfo::{
        UserInfo, UserInfoResponse
    },
    AppState
};
use serde_json::json;


pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    // cfg.service(test_handler);
}