use actix_web::{web, HttpResponse};
use share::common::result::ResponseResult;

pub fn example_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello").route(web::get().to(hello)));
}

async fn hello() -> ResponseResult {
    Ok(HttpResponse::Ok().body("Hello world!"))
}
