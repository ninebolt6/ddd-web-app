use actix_web::web;

use crate::example::presentation::example::example_routes;

pub fn public_routes(cfg: &mut web::ServiceConfig) {
    #[cfg(debug_assertions)]
    example_routes(cfg);
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {}
