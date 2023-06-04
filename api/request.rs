use hermes::respond;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use serde_json::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
struct Payload {
    api_key: String,
    target: String,
    subject: Option<String>,
    app: String,
    ttl: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}


pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    match req.body() {
        Body::Binary(binary_body) =>{
            match String::from_utf8(binary_body.to_owned()) {
                Ok(string_body)=>{
                    match serde_json::from_str::<Payload>(&string_body) {
                        Ok(_data)=>{
                            //Request auth
                            respond(StatusCode::OK, "".to_string())
                        },
                        Err(_)=>{
                            respond(StatusCode::BAD_REQUEST, "".to_string())
                        }
                    }
                },
                Err(_)=>{
                    respond(StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
                }
            }
        },
        _ =>{
            respond(StatusCode::UNSUPPORTED_MEDIA_TYPE, "".to_string())
        }
    }
}
