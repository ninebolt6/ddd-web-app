#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    app::run_server().await
}
