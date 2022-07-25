use actix_web::{get, App, HttpServer};

#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate dotenv;

mod core;
mod schema;

#[get("/health")]
async fn health() -> String {
    "ok".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        App::new().service(
            actix_web::web::scope("/api")
                .service(core::controller::task_controller::get_controller())
                .service(health),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
