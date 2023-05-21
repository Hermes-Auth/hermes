pub mod auth;
pub mod codes;
// pub mod Codes;

use actix_web::{get, HttpResponse, Responder};

#[get("/hermes/health")]
pub async fn get_health() -> impl Responder{
    HttpResponse::Ok().body("Hello world!")
}
