use actix_web::{get, HttpResponse , Responder};
use crate::{
    models::userinfo::{
        UserInfo, UserInfoResponse
    },
    DatabaseState
};
use serde_json::json;


pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    // cfg.service(test_handler);
}