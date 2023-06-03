use reqwest::{self, Client, StatusCode};
use serde::Deserialize;
use std::{collections::HashMap, env::var, format};

#[derive(Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct UserKey {
    pub api_key: String,
}

pub enum PgResult {
    NOPE,
    YESSIR,
    SumnAintRight,
}

pub async fn create_user(email: String) -> bool {
    let api_key = var("SUPABASE_KEY").unwrap();
    let db_url = var("SUPABASE_URL").unwrap();
    let mut body = HashMap::new();
    body.insert("email", email);
    let request = Client::new()
        .post(format!("{db_url}/rest/v1/users"))
        .header("apiKey", api_key)
        .json(&body)
        .send()
        .await;
    if let Ok(response) = request {
        match response.status() {
            StatusCode::CREATED => true,
            _ => false,
        }
    } else {
        false
    }
}

pub async fn user_exists(email: String) -> PgResult {
    let api_key = var("SUPABASE_KEY").unwrap();
    let db_url = var("SUPABASE_URL").unwrap();
    let mut body = HashMap::new();
    body.insert("email", &email);
    let request = Client::new()
        .get(format!("{db_url}/rest/v1/users?email=eq.{email}"))
        .header("apiKey", api_key)
        .send()
        .await;
    if let Ok(response) = request {
        match response.status() {
            StatusCode::OK => {
                if let Ok(body) = response.text().await {
                    if let Ok(data) = serde_json::from_str::<Vec<User>>(&body) {
                        if data.len() == 0 {
                            PgResult::NOPE
                        } else {
                            PgResult::YESSIR
                        }
                    } else {
                        PgResult::SumnAintRight
                    }
                } else {
                    PgResult::SumnAintRight
                }
            }
            _ => PgResult::SumnAintRight,
        }
    } else {
        PgResult::SumnAintRight
    }
}

pub async fn fetch_api_key(email: String) -> (String, bool) {
    let api_key = var("SUPABASE_KEY").unwrap();
    let db_url = var("SUPABASE_URL").unwrap();
    let mut body = HashMap::new();
    body.insert("email", &email);
    let request = Client::new()
        .get(format!("{db_url}/rest/v1/users?select=api_key&email=eq.{email}"))
        .header("apiKey", api_key)
        .send()
        .await;
    if let Ok(response) = request{
        match response.status() {
            StatusCode::OK =>{
                if let Ok(body) = response.text().await {
                    if let Ok(data) = serde_json::from_str::<Vec<UserKey>>(&body){
                        if data.len() == 0 {
                            ("".to_string(), false)
                        }else {
                            (data[0].api_key.to_owned(), true)
                        }
                    }else {
                        ("".to_string(), false)
                    }
                } else {
                    ("".to_string(), false)
                }
            },
            _ =>{
                ("".to_string(), false)
            }
        }
    }else {
        ("".to_string(), false)
    }
}
