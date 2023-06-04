pub mod redis;
pub mod pg;
extern crate lettre;
use std::env::var;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::json;
use vercel_runtime::{Body, Error, Response, StatusCode};
use rand::Rng;

pub enum AuthRequestResult {
    Success,
    ServerErr,
    AppNotFoundErr,
    InvalidKeyErr
}

pub fn send_mail(receiver: String, text: String, subject: String) -> bool {
    let email = Message::builder()
        .from("noreply.auth.hermes@gmail.com".parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(text)
        .unwrap();
    let creds = Credentials::new(
        "noreply.auth.hermes@gmail.com".to_owned(),
        var("SMTP_PASS").unwrap().to_owned(),
    );
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(_) => true,
        Err(err) => {
            println!("Error while sending mail {err}");
            false
        }
    }
}

pub fn respond(status_code: StatusCode) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status_code)
        .body(json!("").to_string().into())?)
}

pub fn generate_code() -> String{
    rand::thread_rng().gen_range(100_000..1_000_000).to_string()
}

pub fn respond_with_body(status_code: StatusCode, data: String) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(json!({"data":data}).to_string().into())?)
}
