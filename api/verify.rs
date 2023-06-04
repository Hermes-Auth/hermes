use hermes::{ respond, redis::{ get_key, del_key }, respond_with_body };
use reqwest::{self, Client};
use serde::Deserialize;
use serde_json::{from_str, Value};
use std::{env::var, println};
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Deserialize)]
struct Payload {
    code: String,
    target: String,
    app_id: String,
    api_key: String,
}

#[derive(Deserialize)]
struct App {
    name: String,
    owner: String,
    default_ttl: String
}

#[derive(Deserialize)]
struct RedisResult {
    result: Value
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match req.body() {
        Body::Binary(binary_body) => match String::from_utf8(binary_body.to_owned()) {
            Ok(string_body) => match from_str::<Payload>(&string_body) {
                Ok(payload) => {
                    let api_key = var("SUPABASE_KEY").unwrap();
                    let db_url = var("SUPABASE_URL").unwrap();
                    let request = Client::new()
                        .get(format!(
                            "{db_url}/rest/v1/apps?id=eq.{}&owner=eq.{}",
                            &payload.app_id, &payload.api_key
                        ))
                        .header("apiKey", api_key)
                        .send()
                        .await;
                    match request {
                        Ok(response)=>{
                            match response.status() {
                                StatusCode::OK =>{
                                    match response.text().await {
                                        Ok(body)=>{
                                            match serde_json::from_str::<Vec<App>>(&body) {
                                                Ok(data)=>{
                                                    if data.len() == 0 {
                                                        respond(StatusCode::UNAUTHORIZED)
                                                    }else {
                                                        let key = format!("code:{}:{}", &payload.app_id, &payload.target);
                                                        match get_key(key.to_owned()).await.as_str() {
                                                            "" => respond(StatusCode::BAD_REQUEST),
                                                            value =>{
                                                                match from_str::<RedisResult>(value) {
                                                                    Ok(value)=>{
                                                                        match value.result {
                                                                            Value::Null=>{
                                                                                respond(StatusCode::BAD_REQUEST)
                                                                            },
                                                                            Value::String(code)=>{
                                                                                if payload.code == code {
                                                                                    let key_deletion = del_key(&key).await;
                                                                                    println!("{key_deletion}");
                                                                                    respond(StatusCode::OK)
                                                                                }else {
                                                                                    respond(StatusCode::BAD_REQUEST)
                                                                                }
                                                                            },
                                                                            _ => respond(StatusCode::INTERNAL_SERVER_ERROR)
                                                                        }
                                                                    },
                                                                    Err(err) => respond_with_body(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
                                                                    
                                                                }
                                                            }
                                                        }
                                                    }
                                                },
                                                Err(_)=> respond(StatusCode::BAD_REQUEST)
                                            }
                                        },
                                        Err(_)=> respond(StatusCode::INTERNAL_SERVER_ERROR)
                                    }
                                },
                                _=> respond(StatusCode::INTERNAL_SERVER_ERROR)
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
