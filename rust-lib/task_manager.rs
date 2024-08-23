use std::env;
use dotenv::dotenv;

struct AppConfig {
    database_url: String,
    api_key: String,
}

impl AppConfig {
    fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let api_key = env::var("API_KEY").expect("API_KEY must be set");

        AppConfig { database_url, api_key }
    }
}

fn main() {
    let config = AppConfig::new();
    println!("Database URL: {}", config.database_url);
    println!("API Key: {}", config.api_key);
}