use hermes::respond;
use std::env::var;
use reqwest::{self, Client };
use serde::Deserialize;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Deserialize)]
struct Payload {
    api_key: String,
    target: String,
    subject: Option<String>,
    app: String,
    ttl: Option<String>,
}

#[derive(Deserialize)]
struct App {
    name: String,
    owner: String,
    default_ttl: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match req.body() {
        Body::Binary(binary_body) => {
            match String::from_utf8(binary_body.to_owned()) {
                Ok(string_body) => {
                    match serde_json::from_str::<Payload>(&string_body) {
                        Ok(payload) => {
                            let api_key = var("SUPABASE_KEY").unwrap();
                            let db_url = var("SUPABASE_URL").unwrap();
                            let request = Client::new()
                                .get(format!("{db_url}/rest/v1/apps?id=eq.{}", &payload.app))
                                .header("apiKey", api_key)
                                .send()
                                .await;
                            match request {
                                Ok(response)=>{
                                    match response.status() {
                                        StatusCode::OK=>{
                                            match response.text().await {
                                                Ok(text)=>{
                                                    match serde_json::from_str::<Vec<App>>(&text) {
                                                        Ok(data)=> {
                                                            if data.len() == 0 {
                                                                respond(StatusCode::UNAUTHORIZED)
                                                            }else {
                                                                respond(StatusCode::OK)
                                                            }
                                                        },
                                                        Err(_)=> respond(StatusCode::BAD_REQUEST)
                                                    }
                                                },
                                                Err(_)=> respond(StatusCode::INTERNAL_SERVER_ERROR)
                                            }
                                        },
                                        _=>respond(StatusCode::BAD_REQUEST)
                                    }
                                },
                                Err(_)=> respond(StatusCode::INTERNAL_SERVER_ERROR)
                            }
                        }
                        Err(_) => respond(StatusCode::BAD_REQUEST)
                    }
                }
                Err(_) => respond(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
        _ => respond(StatusCode::UNSUPPORTED_MEDIA_TYPE),
    }
}
