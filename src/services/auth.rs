use actix_web::{
    get, post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::FromRow;

use crate::utils::{
    gen_api_key,
    redis::{get_key, rm_key, set_key},
};
use crate::AppState;

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct User {
    id: String,
    email: String,
    api_key: String
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
    } else {
    }
    HttpResponse::Ok().body("")
}

#[post("/auth")]
pub async fn register(state: Data<AppState>, body: Json<UserData>) -> impl Responder {
    if let Ok(response) = get_key(body.email.to_string()).await {
        if let Ok(data) = response.text().await {
            let json_data: Result<Value, _> = serde_json::from_str(&data);
            match json_data {
                Ok(value) => {
                    let redis_result = value.get("result").unwrap();
                    match redis_result {
                        Value::Null => {
                            let new_api_key = gen_api_key();
                            match sqlx::query_as::<_, User>("insert into users(email, api_key) values($1, $2) returning id, email, api_key")
                                .bind(&body.email)
                                .bind(new_api_key)
                                .fetch_one(&state.db)
                                .await{
                                    Ok(user)=>{
                                        HttpResponse::Ok().json(json!({"key":user.api_key}))
                                    }
                                    Err(err)=>{
                                        HttpResponse::InternalServerError().body(err.to_string())
                                    }
                                
                            }
                        }
                        Value::String(key) => HttpResponse::Ok().json(json!({ "key": key })),
                        _ => HttpResponse::InternalServerError().body(""),
                    }
                }
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        } else {
            HttpResponse::InternalServerError().body("")
        }
    } else {
        HttpResponse::InternalServerError().body("")
    }
}

#[post("/auth/get_api_key")]
pub async fn login(state: Data<AppState>, body: Json<UserData>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(&body.email)
        .fetch_optional(&state.db)
        .await
    {
        Ok(result) => {
            if let Some(_) = result {
                //Create paseto token
                HttpResponse::Ok().json(json!({"token":"token"}))
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        }
        Err(err) => {
            println!("Error {err}");
            HttpResponse::InternalServerError().body("")
        }
    }
}
