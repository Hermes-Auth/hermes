use std::format;

use hermes::{generate_code, redis::setex_key, respond, send_mail};
use serde::Deserialize;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Deserialize)]
struct Payload {
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if let Body::Binary(binary_body) = req.body() {
        if let Ok(string_body) = String::from_utf8(binary_body.to_owned()) {
            if let Ok(payload) = serde_json::from_str::<Payload>(&string_body) {
                let email = payload.email;
                let code = generate_code();
                if !setex_key(format!("auth:{email}"), code.to_owned(), "300".to_string()).await {
                    respond(
                        StatusCode::INTERNAL_SERVER_ERROR
                    )
                } else {
                    let text =
                        format!("{code} <br> This code will remain valid for the next 5 minutes ");
                    let subject = String::from("Your Hermes authentication code");
                    if send_mail(email, text, subject) {
                        respond(StatusCode::OK)
                    } else {
                        respond(StatusCode::INTERNAL_SERVER_ERROR)
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
