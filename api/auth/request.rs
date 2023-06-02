use hermes::{respond, send_mail, redis::set_key, generate_code};
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
                let text = generate_code();
                let subject = String::from("Your Hermes authentication code");
                if send_mail(email, text, subject) {
                    respond(StatusCode::OK, "".to_string())
                } else {
                    respond(StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
                }
            } else {
                respond(StatusCode::BAD_REQUEST, "".to_string())
            }
        } else {
            respond(StatusCode::BAD_REQUEST, "".to_string())
        }
    } else {
        respond(StatusCode::UNSUPPORTED_MEDIA_TYPE, "".to_string())
    }
}
