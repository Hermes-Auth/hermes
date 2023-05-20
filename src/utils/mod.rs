use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate_unique_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = rng.sample_iter(&Alphanumeric).take(length).collect();
    random_string
}

