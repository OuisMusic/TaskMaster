use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::fmt;

struct AppConfig {
    database_url: String,
    api_key: String,
}

#[derive(Debug)]
struct AppConfigError {
    details: String,
}

impl AppConfigError {
    fn new(msg: &str) -> AppConfigError {
        AppConfigError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AppConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AppConfigError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl AppConfig {
    fn new() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppConfigError::new("DATABASE_URL must be set"))?;
        let api_key = env::var("API_KEY").map_err(|_| AppConfigError::new("API_KEY must be set"))?;

        Ok(AppConfig { database_url, api_key })
    }
}

fn main() {
    match AppConfig::new() {
        Ok(config) => {
            println!("Database URL: {}", config.database_url);
            println!("API Key: {}", config.api_key);
        }
        Err(e) => {
            eprintln!("Application configuration error: {}", e);
            std::process::exit(1);
        }
    }
}