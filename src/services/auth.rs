use actix_web::{
    post, get,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::AppState;
use crate::utils::redis::{ get_key, set_key, rm_key };

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    id: String,
    email: String,
    api_key: String,
}

#[derive(Deserialize)]
pub struct UserData {
    email: String,
}

#[get("/test")]
pub async fn test() -> impl Responder {
    // add code here
    let test = get_key("test".to_string()).await;
    if let Ok(response) = test {
        let breh = response.text().await.unwrap();
        println!("{breh}");
    }else {

    }
    HttpResponse::Ok().body("")
}


#[post("/auth")]
pub async fn register(state: Data<AppState>, body:Json<UserData>) -> impl Responder {
    if let Ok(response) = get_key(body.email.to_string()).await {
        if let Ok(data) = response.text().await{
            match sqlx::query_as::<_, User>("INSERT INTO users(email) VALUES($1) RETURNING id, email, api_key").bind(&body.email).fetch_one(&state.db).await {
                Ok(user)=>{
                    println!("{:?}", user);
                    HttpResponse::Ok().json(json!({"user":"user"}))
                },
                Err(err)=>{
                    match &err {
                        sqlx::Error::Database(error)=>{
                            println!("{error}");
                            HttpResponse::Conflict().body("User already exists")
                        },
                        _=>{
                            print!("{err}");
                            HttpResponse::InternalServerError().body("")
                        }
                    }
                }

            }

        }else {
            HttpResponse::InternalServerError().body("")
        }
    }else{
        HttpResponse::InternalServerError().body("")
    }
}


#[post("/auth/get_api_key")]
pub async fn login(state: Data<AppState>, body: Json<UserData>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(&state.db)
        .await {
            Ok(result)=>{
                if let Some(_) = result{
                    //Create paseto token
                    HttpResponse::Ok().json(json!({"token":"token"}))
                }else{
                    HttpResponse::NotFound().body("User not found")
                }
            },
            Err(err)=>{
                println!("Error {err}");
                HttpResponse::InternalServerError().body("")
            }
        }
}
