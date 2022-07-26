use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, http, App, HttpServer};

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
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new().wrap(Logger::default()).wrap(cors).service(
            actix_web::web::scope("/api")
                .service(core::controller::task_controller::get_controller())
                .service(health),
        )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
