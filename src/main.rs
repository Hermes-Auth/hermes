pub mod services;

use actix_web::{ App, HttpServer, web::Data };
use dotenv::dotenv;
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };

use services::{ get_health };

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

    HttpServer::new( move || {
        App::new()
            .app_data( Data::new(AppState { db: pool.clone() }) )
            .service(get_health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
