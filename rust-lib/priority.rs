use std::env;
use dotenv::dotenv;

struct MyConfig {
    database_url: String,
    api_key: String,
}

impl MyConfig {
    fn new() -> Self {
        dotenv().ok();
        MyConfig {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".into()),
            api_key: env::var("API_KEY").unwrap_or_else(|_| "secret".into()),
        }
    }
}

fn main() {
    let config = MyConfig::new();
    println!("Database URL: {}", config.database_url);
    println!("API Key: {}", config.api_key);
}