mod api;
mod model;

use actix_web::{HttpServer, middleware::Logger, App, web};
use api::auth::{login, sign_in};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(
                web::scope("/auth")
                    .service(login)
                    .service(sign_in)
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
