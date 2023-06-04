use hermes::{
    pg::{create_user, fetch_api_key, user_exists, PgResult},
    redis::get_key,
    respond, respond_with_body,
};
use serde::Deserialize;
use serde_json::Value;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Deserialize)]
struct Payload {
    email: String,
    code: String,
}

#[derive(Deserialize)]
struct RedisValue {
    result: Value,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if let Body::Binary(binary_body) = req.body() {
        if let Ok(string_body) = String::from_utf8(binary_body.to_owned()) {
            if let Ok(payload) = serde_json::from_str::<Payload>(&string_body) {
                match get_key(format!("auth:{}", payload.email)).await.as_str() {
                    "" => respond(StatusCode::BAD_REQUEST),
                    value => {
                        if let Ok(redis_value) = serde_json::from_str::<RedisValue>(value) {
                            match redis_value.result {
                                Value::Null => respond(StatusCode::BAD_REQUEST),
                                Value::String(code) => {
                                    if payload.code == code {
                                        match user_exists(payload.email.to_owned()).await {
                                            PgResult::YESSIR => {
                                                let (data, result) =
                                                    fetch_api_key(payload.email.to_owned()).await;
                                                if result {
                                                    respond_with_body(StatusCode::OK, data)
                                                } else {
                                                    respond(StatusCode::INTERNAL_SERVER_ERROR)
                                                }
                                            }
                                            PgResult::NOPE => {
                                                if create_user(payload.email.to_owned()).await {
                                                    let (data, result) =
                                                        fetch_api_key(payload.email.to_owned())
                                                            .await;
                                                    if result {
                                                        respond_with_body(StatusCode::OK, data)
                                                    } else {
                                                        respond(StatusCode::INTERNAL_SERVER_ERROR)
                                                    }
                                                } else {
                                                    respond(StatusCode::INTERNAL_SERVER_ERROR)
                                                }
                                            }
                                            PgResult::SumnAintRight => {
                                                respond(StatusCode::INTERNAL_SERVER_ERROR)
                                            }
                                        }
                                    } else {
                                        respond(StatusCode::BAD_REQUEST)
                                    }
                                }
                                _ => respond(StatusCode::INTERNAL_SERVER_ERROR),
                            }
                        } else {
                            respond(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
            } else {
                respond(StatusCode::BAD_REQUEST)
            }
        } else {
            respond(StatusCode::BAD_REQUEST)
        }
    } else {
        respond(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }
}
