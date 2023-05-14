use actix_web::{dev::HttpServiceFactory, get, services, HttpResponse};
use share::common::result::ResponseResult;

pub fn example_routes() -> impl HttpServiceFactory {
    services![hello]
}

#[get("/hello")]
async fn hello() -> ResponseResult {
    Ok(HttpResponse::Ok().body("Hello world!"))
}
