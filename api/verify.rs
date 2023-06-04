use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use hermes::respond;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}


pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    respond(StatusCode::OK)
}
