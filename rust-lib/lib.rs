extern crate dotenv;

use std::env;
use std::fmt; // Import the fmt module for custom error formatting.

mod config;
mod tasks;
mod users;

pub use config::Config;
pub use tasks::{Task, TaskManager};
pub use users::{User, UserManager};

// Define a custom error type for your application.
#[derive(Debug)]
enum AppError {
    ConfigError(String),
    TaskManagerError(String),
    UserManagerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::ConfigError(ref err) => write!(f, "Config error: {}", err),
            AppError::TaskManagerError(ref err) => write!(f, "Task Manager error: {}", err),
            AppError::UserManagerError(ref err) => write!(f, "User Manager error: {}", err),
        }
    }
}

// You can also implement Error trait for AppError if needed.

fn init() -> Result<Config, AppError> {
    dotenv::dotenv().ok();

    let config = Config::new().map_err(|e| AppError::ConfigError(e.to_string()))?;

    println!("Productivity system library initialized with config: {:?}", config);
    
    Ok(config)
}

fn main() {
    if let Err(err) = init() {
        println!("Failed to initialize application: {}", err);
        // Depending on your application logic, you might want to exit or take other actions.
        return;
    }

    let task_manager = TaskManager::new().map_err(|e| AppError::TaskManagerError(e.to_string()));
    if let Err(err) = task_manager {
        println!("Failed to create a task manager: {}", err);
        return;
    }

    let user_manager = UserManager::new().map_err(|e| AppError::UserManagerError(e.to_string()));
    if let Err(err) = user_manager {
        println!("Failed to create a user manager: {}", err);
        return;
    }

    // Assuming creating a task or user might fail, wrap these calls in error handling too.
    let _task = task_manager.unwrap().create_task("Finish Rust project", "Due in two weeks");
    let _user = user_manager.unwrap().create_user("John Doe", "john@example.com");

    println!("Main application logic goes here.");
}