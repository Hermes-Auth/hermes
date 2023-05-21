use reqwest::Client;

struct Config {
    url: String,
    token: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: std::env::var("REDIS_URL").expect(""),
            token: std::env::var("REDIS_TOKEN").expect(""),
        }
    }
}

async fn req_redis( command:String ) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("{}/{}", Config::default().url, command);
    Client::new()
        .get(url)
        .header("Authorization", Config::default().token)
        .send()
        .await
}

pub async fn get_key( key: String ) -> Result<reqwest::Response, reqwest::Error>  {
    let command = format!("get/{key}");
    req_redis(command).await
}

pub async fn set_key() {}

pub async fn rm_key() {}
