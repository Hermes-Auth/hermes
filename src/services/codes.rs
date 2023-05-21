extern crate lettre;
use std::print;

use actix_web::{get, HttpResponse, Responder};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[get("/code/request")]
pub async fn send_code() -> impl Responder {
    let email = Message::builder()
        .from("noreply.auth.hermes@gmail.com".parse().unwrap())
        .to("zozozozeph@gmail.com".parse().unwrap())
        .subject("Your authentication code")
        .header(ContentType::TEXT_PLAIN)
        .body("69".to_string())
        .unwrap();
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("");
    let creds = Credentials::new("noreply.auth.hermes@gmail.com".to_string(), smtp_password);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(_)=>{
            print!("Your code was sent souksexfouly")
        },
        Err(err)=>{
            print!("Something bad happened {err}")
        }
    }
    HttpResponse::Ok()
}
