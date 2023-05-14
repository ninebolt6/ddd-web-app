pub mod route;

#[cfg(debug_assertions)]
pub mod example;

use actix_web::{web, App, HttpServer};

pub async fn run_server() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").configure(route::public_routes).service(
                web::scope("")
                    // .wrap(auth_middleware)
                    .configure(route::auth_routes),
            ),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
