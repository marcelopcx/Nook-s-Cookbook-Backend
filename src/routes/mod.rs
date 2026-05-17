use actix_web::web;

use crate::handlers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::health_check)
        .service(handlers::login)
        .service(handlers::register)
        .service(handlers::get_me)
        .service(handlers::patch_me);
}