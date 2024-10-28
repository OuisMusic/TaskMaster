use std::env;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct TaskMasterError {
    message: String,
}

impl TaskMasterError {
    fn new(message: &str) -> TaskMasterError {
        TaskMasterError {
            message: message.to_string(),
        }
    }
}

impl Display for TaskMasterError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TaskMasterError {}

fn fetch_environment_variable(variable_name: &str) -> Result<String, TaskMasterError> {
    env::var(variable_name).map_err(|_| TaskMasterError::new(&format!("Missing environment variable: {}", variable_name)))
}

fn ensure_non_empty_string(input: &str) -> Result<(), TaskMasterError> {
    if input.trim().is_empty() {
        Err(TaskMasterError::new("Input cannot be empty."))
    } else {
        Ok(())
    }
}

fn transform_date_to_standard_format(date_str: &str) -> Result<String, TaskMasterError> {
    let date_components: Vec<&str> = date_str.split('-').collect();
    if date_components.len() != 3 {
        Err(TaskMasterError::new("Date must be in the format YYYY-MM-DD."))
    } else {
        Ok(format!("{}/{}/{}", date_components[2], date_components[1], date_components[0]))
    }
}

fn main() {
    match fetch_environment_variable("EXAMPLE_ENV_VAR") {
        Ok(value) => println!("Environment variable value: {}", value),
        Err(error) => println!("{}", error),
    }

    match ensure_non_empty_string("Test") {
        Ok(_) => println!("Validation passed."),
        Err(error) => println!("{}", error),
    }

    match transform_date_to_standard_format("2023-01-02") {
        Ok(formatted_date) => println!("Formatted date: {}", formatted_date),
        Err(error) => println!("{}", error),
    }
}