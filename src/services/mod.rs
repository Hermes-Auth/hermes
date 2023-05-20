use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/hermes/health")]
pub async fn get_health() -> impl Responder{
    HttpResponse::Ok().body("Hello world!")
}
