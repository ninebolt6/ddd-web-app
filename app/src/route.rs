use actix_web::web;

use crate::example::presentation::example::example_routes;

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    #[cfg(debug_assertions)]
    cfg.service(example_routes());
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {}
