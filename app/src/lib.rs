pub mod route;

#[cfg(debug_assertions)]
pub mod example;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use shared::external::database::{connect_db, ConnectionFactoryImpl};

pub async fn run_server() -> Result<(), std::io::Error> {
    let pool = connect_db().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api").configure(route::public_routes).service(
                    web::scope("")
                        // .wrap(auth_middleware)
                        .configure(route::auth_routes),
                ),
            )
            .app_data(Data::new(ConnectionFactoryImpl::new(pool.clone())))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
