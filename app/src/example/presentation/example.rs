use actix_web::{dev::HttpServiceFactory, get, services, HttpResponse, Result};

pub fn example_routes() -> impl HttpServiceFactory {
    services![hello]
}

#[get("/hello")]
async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}
