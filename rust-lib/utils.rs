use std::env;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct UtilityError {
    message: String,
}

impl UtilityError {
    fn new(message: &str) -> UtilityError {
        UtilityError {
            message: message.to_string(),
        }
    }
}

impl Display for UtilityError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UtilityError {}

fn get_env_var(key: &str) -> Result<String, UtilityError> {
    env::var(key).map_err(|_| UtilityError::new(&format!("Missing env variable: {}", key)))
}

fn validate_non_empty(data: &str) -> Result<(), UtilityError> {
    if data.trim().is_empty() {
        Err(UtilityError::new("Data cannot be empty."))
    } else {
        Ok(())
    }
}

fn format_date(date: &str) -> Result<String, UtilityError> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        Err(UtilityError::new("Date must be in the format YYYY-MM-DD."))
    } else {
        Ok(format!("{}/{}/{}", parts[2], parts[1], parts[0]))
    }
}

fn main() {
    match get_env_var("EXAMPLE_ENV_VAR") {
        Ok(value) => println!("Environment variable value: {}", value),
        Err(e) => println!("{}", e),
    }

    match validate_non_empty("Test") {
        Ok(_) => println!("Validation passed."),
        Err(e) => println!("{}", e),
    }

    match format_date("2023-01-02") {
        Ok(formatted_date) => println!("Formatted date: {}", formatted_date),
        Err(e) => println!("{}", e),
    }
}