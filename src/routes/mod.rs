pub mod checks;
pub mod test;
pub mod userinfo;

use crate::Config;
use actix_web::{web, guard, HttpRequest,HttpResponse};


pub fn core_route(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api")
                .guard(guard::Post())
                .guard(guard::Header("YXGames", "BuyTheFullVersion"))
                .configure(test::configure)
                .configure(checks::configure)
        )
        .service(
            web::scope("/api/auth")
                .guard(guard::Post())
                .guard(guard::Header("YXGames", "BuyTheFullVersion"))
                .configure(userinfo::configure)
        );
}