use hermes::{redis::get_key, respond};
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
                    "" => respond(
                        StatusCode::BAD_REQUEST,
                        "Something went wrong while verifying your code".to_string(),
                    ),
                    value => {
                        if let Ok(redis_value) = serde_json::from_str::<RedisValue>(value) {
                            match redis_value.result {
                                Value::Null => {
                                    respond(StatusCode::BAD_REQUEST, "Invalid code. Code not found".to_string())
                                }
                                Value::String(code) => {
                                    if payload.code == code {
                                        respond(StatusCode::OK, "".to_string())
                                    } else {
                                        respond(StatusCode::BAD_REQUEST, format!("Invalid code {code} {}", payload.code))
                                    }
                                }
                                _ => respond(
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    "Something went wrong".to_string(),
                                ),
                            }
                        } else {
                            respond(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "Failed to parse redis data".to_string(),
                            )
                        }
                    }
                }
            } else {
                respond(StatusCode::BAD_REQUEST, "Failed to parse body".to_string())
            }
        } else {
            respond(StatusCode::BAD_REQUEST, "".to_string())
        }
    } else {
        respond(StatusCode::UNSUPPORTED_MEDIA_TYPE, "".to_string())
    }
}
