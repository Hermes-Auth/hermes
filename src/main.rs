pub mod services;
pub mod utils;

use actix_web::{ App, HttpServer, web::Data, middleware::Logger };
use dotenv::dotenv;
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };

use services::{ get_health, codes::send_code, auth::{register, login, test} };

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DB_URL").expect("");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");
    println!("Connected to database");
    HttpServer::new( move || {
        App::new()
            .wrap(Logger::default())
            .app_data( Data::new(AppState { db: pool.clone() }) )
            .service(get_health)
            .service(send_code)
            .service(register)
            .service(login)
            .service(test)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
