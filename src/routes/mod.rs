pub mod checks;
pub mod test;
pub mod userinfo;

use crate::{ConfigState, DatabaseState};
use actix_web::{
    guard::{self, Guard},
    web, Error, HttpRequest, HttpResponse,
};
use std::sync::Arc;

pub fn core_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .guard(guard::Post())
            .configure(test::configure)
            .configure(checks::configure)
            .configure(userinfo::configure),
    );
}
