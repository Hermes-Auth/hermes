use hermes::{
    pg::{create_app, CreateAppResult},
    respond,
};
use serde::Deserialize;
use serde_json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

#[derive(Deserialize)]
struct App {
    name: String,
    owner: String,
    default_ttl: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    if let Body::Binary(binary_body) = req.body() {
        if let Ok(string_body) = String::from_utf8(binary_body.to_owned()) {
            if let Ok(payload) = serde_json::from_str::<App>(&string_body) {
                match create_app(
                    &payload.name,
                    &payload.owner.to_owned(),
                    &payload.default_ttl,
                )
                .await
                {
                    CreateAppResult::OK => respond(StatusCode::OK),
                    CreateAppResult::CONFLICT => respond(StatusCode::CONFLICT),
                    _ => respond(StatusCode::INTERNAL_SERVER_ERROR),
                }
            } else {
                respond(StatusCode::BAD_REQUEST)
            }
        } else {
            respond(StatusCode::INTERNAL_SERVER_ERROR)
        }
    } else {
        respond(StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }
}
