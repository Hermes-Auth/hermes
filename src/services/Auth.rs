use actix_web::{web::{ Data, Json }, post, HttpResponse, Responder};
use serde::{ Serialize, Deserialize };
use sqlx::FromRow;

use crate::AppState;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User{
    id: String,
    email: String,
    api_key: String
}

#[derive(Deserialize)]
pub struct UserData {
    email: String
}

#[post("/auth/register")]
pub async fn register( state: Data<AppState>, body: Json<UserData> ) -> impl Responder {
    match sqlx::query_as::<_, User>("INSERT INTO users (email) VALUES ($1)").bind(body.email.to_string()).fetch_one(&state.db).await{
        Ok(user)=>{
            HttpResponse::Ok().json(user)
        },
        Err(e)=>{
            print!("{e}");
            HttpResponse::InternalServerError().body("")
        }
    }
}

#[post("/auth/login")]
pub async fn login( state: Data<AppState>, body: Json<User> ) -> impl Responder {

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 ").bind(body.email.to_string()).fetch_one(&state.db).await {
        Ok(user)=>{
            HttpResponse::Ok().json(user)
        },
        Err(err)=>{
            print!("{err}");
            HttpResponse::InternalServerError().body("")
        }
    }

}
