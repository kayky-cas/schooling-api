mod api;
mod model;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{auth, school};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    dotenv().ok();

    let datatbase_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set!");

    let app_state = AppState {
        db: PgPoolOptions::new()
            .max_connections(5)
            .connect(&datatbase_url)
            .await
            .expect("Error building the connection pool!"),
    };

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new()
            .app_data(Data::new(app_state.clone()))
            .wrap(logger)
            .service(auth())
            .service(school())
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
