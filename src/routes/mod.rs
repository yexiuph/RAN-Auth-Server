pub mod checks;
pub mod test;
pub mod userinfo;

use actix_web::{web, guard::{self, Guard}, HttpRequest, HttpResponse, Error};
use crate::{DatabaseState, ConfigState};
use std::sync::Arc;

pub fn core_route(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api")
                .guard(guard::Post())
                .guard(guard::Header("YXGames", "CoreValidator"))
                .configure(test::configure)
                .configure(checks::configure),
        )
        .service(
            web::scope("/api/auth")
                .guard(guard::Post())
                .guard(guard::Header("YXGames", "CoreValidator"))
                .configure(userinfo::configure),
        );
}