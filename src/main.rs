use std::io::Result;

use actix_web::{App, HttpServer};
use sqlx::postgres::PgPoolOptions;

pub mod game;
pub mod index;

// Environment variable for db connection
const DATABASE_URL: &str = "postgres://poker:password@localhost:5432/poker";

#[actix_web::main]
async fn main() -> Result<()> {
    // let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(DATABASE_URL).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(index::xindex)
            .service(index::xdeal_hand)
    })
    .bind("localhost:8080")
    .expect("Must be able to bind server")
    .run()
    .await
}
