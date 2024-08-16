extern crate dotenv;

use std::env;

mod config;
mod tasks;
mod users;

pub use config::Config;
pub use tasks::{Task, TaskManager};
pub use users::{User, UserManager};

fn init() {
    dotenv::dotenv().ok();

    let config = Config::new();

    println!("Productivity system library initialized with config: {:?}", config);
}

fn main() {
    init();

    let task_manager = TaskManager::new();
    let user_manager = UserManager::new();

    let _task = task_manager.create_task("Finish Rust project", "Due in two weeks");
    let _user = user_manager.create_user("John Doe", "john@example.com");

    println!("Main application logic goes here.");
}