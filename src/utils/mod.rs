use rand::{ distributions::Alphanumeric, Rng };

pub mod redis;

pub fn gen_api_key() -> String{
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
