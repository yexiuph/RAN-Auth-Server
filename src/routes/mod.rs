pub mod checks;
pub mod test;
pub mod userinfo;

pub fn core_route(cfg: &mut actix_web::web::ServiceConfig) {
    checks::configure(cfg);
    test::configure(cfg);
    userinfo::configure(cfg);
}