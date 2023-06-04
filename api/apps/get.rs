use hermes::{respond, respond_with_body};
use serde::Deserialize;
use serde_json::from_str;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use reqwest::{self, Client};
use std::env::var;

#[derive(Deserialize)]
struct Payload {
    api_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match req.body() {
        Body::Binary(bin) => match String::from_utf8(bin.to_owned()) {
            Ok(string_body) => match from_str::<Payload>(&string_body) {
                Ok(payload) => {
                    let api_key = var("SUPABASE_KEY").unwrap();
                    let db_url = var("SUPABASE_URL").unwrap();
                    let request = Client::new()
                        .get(format!("{db_url}/rest/v1/apps?owner=eq.{}", payload.api_key))
                        .header("apiKey", api_key)
                        .send()
                        .await;
                    match request {
                        Ok(response)=>{
                            match response.status() {
                                StatusCode::OK =>{
                                    let data = response.text().await;
                                    match data {
                                        Ok(apps)=>{
                                            respond_with_body(StatusCode::OK, apps)
                                        },
                                        Err(_)=> respond(StatusCode::INTERNAL_SERVER_ERROR)
                                    }
                                },
                                _ => respond(StatusCode::INTERNAL_SERVER_ERROR)
                            }
                        },
                        Err(_)=> respond(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
                Err(_) => respond(StatusCode::BAD_REQUEST),
            },
            Err(_) => respond(StatusCode::INTERNAL_SERVER_ERROR),
        },
        _ => respond(StatusCode::UNSUPPORTED_MEDIA_TYPE),
    }
}
